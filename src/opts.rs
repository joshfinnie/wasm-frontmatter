use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Opt {
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
    pub fn new() -> Self {
        Opt {
            delimiters: Some(String::from("---")),
            excerpt: Some(false),
            excerpt_separator: Some(String::from("---")),
        }
    }
    pub fn extract_options(self) -> (String, bool, String) {
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
