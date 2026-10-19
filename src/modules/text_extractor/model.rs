#![allow(dead_code)]

/// Text manipulation and formatting utilities for Text Extractor
/// Ported from Caelestia Plugins / text-extractor / TextUtils.js

pub fn clean_whitespace(text: &str) -> String {
    text.trim().to_string()
}

pub fn to_single_line(text: &str) -> String {
    let replaced = text.replace(['\r', '\n'], " ");
    let mut result = String::with_capacity(replaced.len());
    let mut prev_space = true; // trims leading spaces

    for ch in replaced.chars() {
        if ch == ' ' || ch == '\t' {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(ch);
            prev_space = false;
        }
    }

    if result.ends_with(' ') {
        result.pop();
    }
    result
}

pub fn clean_lines(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<&str>>()
        .join("\n")
        .trim()
        .to_string()
}

pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

pub fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

pub fn to_title_case(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut capitalize_next = true;

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            if capitalize_next {
                result.extend(ch.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(ch.to_lowercase());
            }
        } else {
            result.push(ch);
            if ch.is_whitespace() || ch == '-' || ch == '_' {
                capitalize_next = true;
            }
        }
    }
    result
}

pub fn to_markdown_code(text: &str, lang: Option<&str>) -> String {
    let l = lang.unwrap_or("");
    format!("```{}\n{}\n```", l, text)
}

pub fn to_json_string(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_else(|_| format!("\"{}\"", text))
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

pub fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

pub fn snippet(text: &str, max_len: usize) -> String {
    let clean = to_single_line(text);
    let chars: Vec<char> = clean.chars().collect();
    let limit = if max_len == 0 { 60 } else { max_len };
    if chars.len() <= limit {
        clean
    } else {
        let truncated: String = chars[..limit].iter().collect();
        format!("{}...", truncated)
    }
}

pub fn format_text(format_type: &str, text: &str) -> String {
    match format_type {
        "RAW" => text.to_string(),
        "SINGLE_LINE" => to_single_line(text),
        "MARKDOWN" => to_markdown_code(text, None),
        "UPPERCASE" => to_uppercase(text),
        "LOWERCASE" => to_lowercase(text),
        "TITLECASE" => to_title_case(text),
        "JSON" => to_json_string(text),
        _ => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line() {
        let input = "Hello   world\n\nthis is\ta   test\n";
        assert_eq!(to_single_line(input), "Hello world this is a test");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("OCR text extractor"), "Ocr Text Extractor");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(count_words("one two three  four "), 4);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_snippet() {
        let input = "A quick brown fox jumps over the lazy dog";
        assert_eq!(snippet(input, 15), "A quick brown f...");
    }
}
