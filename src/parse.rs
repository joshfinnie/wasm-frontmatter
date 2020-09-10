pub fn extract_frontmatter(markdown_input: &str, delimiter: String) -> Option<&str> {
    let beginning_frontmatter = &markdown_input[4..];
    beginning_frontmatter.find(&format!("{}\n", delimiter))?;
    let splits: Vec<&str> = beginning_frontmatter.split(&delimiter).collect();
    if splits.is_empty() {
        return None;
    };
    Some(splits[0])
}

pub fn parse_frontmatter(markdown_input: &str, delimiters: String) -> (serde_yaml::Value, &str) {
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

pub fn parse_excerpt(markdown_input: &str, separator: String) -> &str {
    let splits: Vec<&str> = markdown_input.split(&separator).collect();
    if splits.len() == 1 {
        return "";
    };
    splits[0]
}

