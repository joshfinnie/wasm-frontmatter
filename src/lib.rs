mod utils;

use serde::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn extract_frontmatter(markdown_input: &str) -> Option<String> {
    let beginning_frontmatter = &markdown_input[4..];
    let fm_end = beginning_frontmatter.find("---\n");
    if fm_end.is_none() {
        return None;
    };
    let splits: Vec<&str> = beginning_frontmatter.split("---").collect();
    if splits.is_empty() {
        return None;
    };
    Some(splits[0].to_string())
}

fn parse_frontmatter(markdown_input: &str) -> (serde_yaml::Value, &str) {
    let fm = match markdown_input.starts_with("---\n") {
        true => extract_frontmatter(markdown_input),
        false => None
    };

    match fm {
        None => (serde_yaml::from_str("{}").unwrap(), markdown_input),
        Some(data) => {
            let frontmatter_length = data.chars().count() + 8;
            (serde_yaml::from_str(&data).unwrap(), &markdown_input[frontmatter_length..])
        }
    }
}

fn parse_excerpt(markdown_input: &str) -> Option<&str> {
    let splits: Vec<&str> = markdown_input.split("---").collect();
    if splits.is_empty() {
        return None;
    };
    Some(splits[0])
}

#[derive(Serialize)]
pub struct Data {
    content: String,
    data: serde_yaml::Value,
    excerpt: String,
    //isEmpty: bool,
}

#[wasm_bindgen]
pub fn matter(markdown_input: &str) -> JsValue {
    utils::set_panic_hook();

    let (frontmatter, content) = parse_frontmatter(markdown_input);
    let excerpt = parse_excerpt(content);
    let data = Data{
        content: content.to_string(),
        data: frontmatter,
        excerpt: match excerpt {
            None => "".to_string(),
            Some(data) => data.to_string()
        }
    };

    JsValue::from_serde(&data).unwrap()
}

#[test]
fn test_parse_frontmatter() {
    let test_str = "---\ntitle: Home\n---\nOther stuff";
    let (fm, content) = parse_frontmatter(test_str);

    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\ntitle: Home");
    assert_eq!(content.chars().count(), 11);
}

#[test]
fn test_parse_frontmatter_with_excerpt() {
    let test_str = r#"---
foo: bar
---
This is an excerpt.
---
This is content"#;
    let (fm, content) = parse_frontmatter(test_str);
    let excerpt = parse_excerpt(content);
    assert_eq!(serde_yaml::to_string(&fm).unwrap(), "---\nfoo: bar");
    assert_eq!(excerpt.unwrap_or("").chars().count(), 20);
}
