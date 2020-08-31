mod utils;

use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn extract_frontmatter(markdown_input: &str, delimiter: String) -> Option<String> {
    let beginning_frontmatter = &markdown_input[4..];
    let fm_end = beginning_frontmatter.find(&format!("{}\n", delimiter));
    if fm_end.is_none() {
        return None;
    };
    let splits: Vec<&str> = beginning_frontmatter.split(&delimiter).collect();
    if splits.is_empty() {
        return None;
    };
    Some(splits[0].to_string())
}

fn parse_frontmatter(
    markdown_input: &str,
    delimiters: Option<String>,
) -> (serde_yaml::Value, &str) {
    let delimiter = delimiters.unwrap_or("---".to_string());
    let fm = match markdown_input.starts_with(&format!("{}\n", delimiter)) {
        true => extract_frontmatter(markdown_input, delimiter),
        false => None,
    };

    match fm {
        None => (serde_yaml::from_str("{}").unwrap(), markdown_input),
        Some(data) => {
            let frontmatter_length = data.chars().count() + 8;
            (
                serde_yaml::from_str(&data).unwrap(),
                &markdown_input[frontmatter_length..],
            )
        }
    }
}

fn parse_excerpt(markdown_input: &str, separator: String) -> String {
    let splits: Vec<&str> = markdown_input.split(&separator).collect();
    if splits.len() == 1 {
        return "".to_string();
    };
    splits[0].to_string()
}

#[derive(Serialize)]
struct Data {
    content: String,
    data: serde_yaml::Value,
    excerpt: String,
    //isEmpty: bool,
}

#[derive(Serialize, Deserialize)]
struct Opt {
    delimiters: Option<String>,
    excerpt: Option<bool>,
    excerpt_separator: Option<String>,
}

impl Opt {
    fn new() -> Self {
        Opt {
            delimiters: Some("---".to_string()),
            excerpt: Some(false),
            excerpt_separator: Some("---".to_string()),
        }
    }
    fn extract_options(self) -> (Option<String>, bool, String) {
        let delimiters = self.delimiters.clone();
        let mut excerpt = self.excerpt.clone().unwrap_or(false);
        let excerpt_separator = self.excerpt_separator.clone().unwrap_or("---".to_string());
        if excerpt_separator != "---".to_string() {
            excerpt = true;
        }

        (delimiters, excerpt, excerpt_separator)
    }
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.delimiters.clone().unwrap_or("---".to_string()),
            self.excerpt.unwrap_or(false),
            self.excerpt_separator.clone().unwrap_or("---".to_string()),
        )
    }
}

#[wasm_bindgen]
pub fn matter(markdown_input: &str, opt: JsValue) -> JsValue {
    utils::set_panic_hook();

    let options: Opt = match opt.is_object() {
        true => opt.into_serde().unwrap(),
        false => Opt::new(),
    };

    let (delimiters, excerpt, excerpt_separator) = options.extract_options();
    let (frontmatter, content) = parse_frontmatter(markdown_input, delimiters);
    let data = Data {
        content: content.to_string(),
        data: frontmatter,
        excerpt: match excerpt {
            true => parse_excerpt(content, excerpt_separator),
            false => "".to_string(),
        },
    };

    JsValue::from_serde(&data).unwrap()
}

#[test]
fn test_parse_frontmatter() {
    let test_str = "---\ntitle: Home\n---\nOther stuff";
    let (fm, content) = parse_frontmatter(test_str, None);

    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\ntitle: Home");
    assert_eq!(content.chars().count(), 11);
}

#[test]
fn test_parse_frontmatter_with_custom_delimiter() {
    let test_str = "~~~\ntitle: Home\n~~~\nOther stuff";
    let (fm, content) = parse_frontmatter(test_str, Some("~~~".to_string()));

    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\ntitle: Home");
    assert_eq!(content.chars().count(), 11);
}

#[test]
fn test_extract_options() {
    let opt = Opt::new();
    let e = opt.extract_options();
    assert_eq!(e, (Some("---".to_string()), false, "---".to_string()))
}

#[test]
fn test_parse_excerpt() {
    let markdown = "this is an excerpt\n---\nthis is content.";
    let excerpt = parse_excerpt(markdown, "---".to_string());
    assert_eq!(excerpt, "this is an excerpt\n".to_string());
}

#[test]
fn test_parse_excerpt_no_excerpt() {
    let markdown = "this is just content, no excerpt.";
    let excerpt = parse_excerpt(markdown, "---".to_string());
    assert_eq!(excerpt, "".to_string());
}

#[test]
fn test_parse_excerpt_with_custome_delimiter() {
    let markdown = "this is an excerpt\n<!-- end -->\nthis is content.";
    let excerpt = parse_excerpt(markdown, "<!-- end -->".to_string());
    assert_eq!(excerpt, "this is an excerpt\n".to_string());
}

#[test]
fn test_parse_frontmatter_with_excerpt() {
    let test_str = r#"---
foo: bar
---
This is an excerpt.
---
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, None);
    let options = Opt::new();
    let excerpt = parse_excerpt(content, options.excerpt_separator.unwrap());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\nfoo: bar");
    assert_eq!(excerpt.chars().count(), 20);
}

#[test]
fn test_parse_frontmatter_with_custom_excerpt() {
    let test_str = r#"---
foo: bar
---
This is a long excerpt with custom separator.
<!-- end -->
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, None);
    let options = Opt {
        delimiters: Some("---".to_string()),
        excerpt: Some(true),
        excerpt_separator: Some("<!-- end -->".to_string()),
    };
    let excerpt = parse_excerpt(content, options.excerpt_separator.unwrap());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\nfoo: bar");
    assert_eq!(excerpt.chars().count(), 46);
}

#[test]
fn test_parse_frontmatter_with_no_frontmatter() {
    let test_str = r#"This is a long excerpt with custom separator.
---
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, None);
    let options = Opt::new();
    let excerpt = parse_excerpt(content, options.excerpt_separator.unwrap());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\n{}");
    assert_eq!(excerpt.chars().count(), 46);
}

#[test]
fn test_parse_frontmatter_with_custom_excerpt_no_frontmatter() {
    let test_str = r#"This is a long excerpt with custom separator.
<!-- end -->
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, None);
    let options = Opt {
        delimiters: Some("---".to_string()),
        excerpt: Some(true),
        excerpt_separator: Some("<!-- end -->".to_string()),
    };
    let excerpt = parse_excerpt(content, options.excerpt_separator.unwrap());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\n{}");
    assert_eq!(excerpt.chars().count(), 46);
}
