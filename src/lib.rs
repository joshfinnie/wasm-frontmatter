mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn extract_frontmatter<'a>(markdown_input: &'a str, delimiter: &'a str) -> Option<&'a str> {
    let beginning_frontmatter = &markdown_input[4..];
    beginning_frontmatter.find(&format!("{}\n", delimiter))?;
    let splits: Vec<&str> = beginning_frontmatter.split(&delimiter).collect();
    if splits.is_empty() {
        return None;
    };
    Some(splits[0])
}

fn parse_frontmatter<'a>(
    markdown_input: &'a str,
    delimiters: &'a str,
) -> (serde_yaml::Value, &'a str) {
    if markdown_input.starts_with(&format!("{}\n", delimiters)) {
        return match extract_frontmatter(markdown_input, delimiters) {
            None => (serde_yaml::from_str("{}").unwrap(), markdown_input),
            Some(data) => {
                let frontmatter_length = data.chars().count() + 8;
                (
                    serde_yaml::from_str(&data).unwrap(),
                    &markdown_input[frontmatter_length..],
                )
            }
        };
    }
    (serde_yaml::from_str("{}").unwrap(), markdown_input)
}

fn parse_excerpt<'a>(markdown_input: &'a str, separator: &'a str) -> &'a str {
    let splits: Vec<&str> = markdown_input.split(&separator).collect();
    if splits.len() == 1 {
        return "";
    };
    splits[0]
}

#[derive(Serialize)]
struct Output<'a> {
    content: &'a str,
    data: serde_yaml::Value,
    excerpt: &'a str,
    //isEmpty: bool,
}

#[derive(Serialize, Deserialize)]
struct Opt {
    delimiters: Option<String>,
    excerpt: Option<bool>,
    excerpt_separator: Option<String>,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            delimiters: Some(String::from("---")),
            excerpt: Some(false),
            excerpt_separator: Some(String::from("---")),
        }
    }
}

impl Opt {
    fn new() -> Self {
        Opt {
            delimiters: Some(String::from("---")),
            excerpt: Some(false),
            excerpt_separator: Some(String::from("---")),
        }
    }
    fn extract_options(self) -> (String, bool, String) {
        let delimiters = match self.delimiters {
            None => String::from("---"),
            Some(data) => data,
        };
        let mut excerpt = match self.excerpt {
            None => false,
            Some(data) => data,
        };
        let excerpt_separator = match self.excerpt_separator {
            None => String::from("---"),
            Some(data) => data,
        };
        if excerpt_separator != "---" {
            excerpt = true;
        }

        (delimiters, excerpt, excerpt_separator)
    }
}

#[wasm_bindgen]
pub fn matter(markdown_input: &str, opt: JsValue) -> JsValue {
    utils::set_panic_hook();

    let options: Opt = match opt.into_serde() {
        Ok(data) => data,
        _ => Opt::new(),
    };

    let (delimiters, excerpt, excerpt_separator) = options.extract_options();
    let (data, content) = parse_frontmatter(markdown_input, delimiters.as_str());
    let output = Output {
        content,
        data,
        excerpt: match excerpt {
            true => parse_excerpt(content, excerpt_separator.as_str()),
            false => "",
        },
    };

    JsValue::from_serde(&output).unwrap()
}

#[test]
fn test_parse_frontmatter() {
    let test_str = "---\ntitle: Home\n---\nOther stuff";
    let (fm, content) = parse_frontmatter(test_str, "---");

    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\ntitle: Home\n");
    assert_eq!(content.chars().count(), 11);
}

#[test]
fn test_parse_frontmatter_with_custom_delimiter() {
    let test_str = "~~~\ntitle: Home\n~~~\nOther stuff";
    let (fm, content) = parse_frontmatter(test_str, "~~~");

    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\ntitle: Home\n");
    assert_eq!(content.chars().count(), 11);
}

#[test]
fn test_extract_options() {
    let opt = Opt::new();
    let e = opt.extract_options();
    assert_eq!(e, ("---".to_string(), false, "---".to_string()))
}

#[test]
fn test_parse_excerpt() {
    let markdown = "this is an excerpt\n---\nthis is content.";
    let excerpt = parse_excerpt(markdown, "---");
    assert_eq!(excerpt, "this is an excerpt\n");
}

#[test]
fn test_parse_excerpt_no_excerpt() {
    let markdown = "this is just content, no excerpt.";
    let excerpt = parse_excerpt(markdown, "---");
    assert_eq!(excerpt, "");
}

#[test]
fn test_parse_excerpt_with_custome_delimiter() {
    let markdown = "this is an excerpt\n<!-- end -->\nthis is content.";
    let excerpt = parse_excerpt(markdown, "<!-- end -->");
    assert_eq!(excerpt, "this is an excerpt\n");
}

#[test]
fn test_parse_frontmatter_with_excerpt() {
    let test_str = r#"---
foo: bar
---
This is an excerpt.
---
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, "---");
    let options = Opt::new();
    let excerpt_separator = options.excerpt_separator.unwrap();
    let excerpt = parse_excerpt(content, excerpt_separator.as_str());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\nfoo: bar\n");
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
    let (fm, content) = parse_frontmatter(test_str, "---");
    let options = Opt {
        delimiters: Some("---".to_string()),
        excerpt: Some(true),
        excerpt_separator: Some("<!-- end -->".to_string()),
    };
    let excerpt_separator = options.excerpt_separator.unwrap();
    let excerpt = parse_excerpt(content, excerpt_separator.as_str());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\nfoo: bar\n");
    assert_eq!(excerpt.chars().count(), 46);
}

#[test]
fn test_parse_frontmatter_with_no_frontmatter() {
    let test_str = r#"This is a long excerpt with custom separator.
---
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, "---");
    let options = Opt::new();
    let excerpt_separator = options.excerpt_separator.unwrap();
    let excerpt = parse_excerpt(content, excerpt_separator.as_str());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\n{}\n");
    assert_eq!(excerpt.chars().count(), 46);
}

#[test]
fn test_parse_frontmatter_with_custom_excerpt_no_frontmatter() {
    let test_str = r#"This is a long excerpt with custom separator.
<!-- end -->
This is content"#;
    let (fm, content) = parse_frontmatter(test_str, "---");
    let options = Opt {
        delimiters: Some("---".to_string()),
        excerpt: Some(true),
        excerpt_separator: Some("<!-- end -->".to_string()),
    };
    let excerpt_separator = options.excerpt_separator.unwrap();
    let excerpt = parse_excerpt(content, excerpt_separator.as_str());
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\n{}\n");
    assert_eq!(excerpt.chars().count(), 46);
}
