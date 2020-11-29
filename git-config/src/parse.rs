use regex::Regex;
use std::str::Split;

// https://git-scm.com/docs/git-config#_syntax

// section starts with [ and ends with ]
// section is alphanumeric (ASCII) with - and .
// section is case insensitive
// subsection is optional
// subsection is specified after section and one or more spaces
// subsection is specified between double quotes
const SECTION_LINE_REGEX: Regex = Regex::new(r#"^\[([A-Za-z0-9-.]+)(?: "(.*)")?\]$"#).unpack();
const SECTION_REGEX: Regex = Regex::new(r#"^[A-Za-z0-9-.]+$"#).unpack();

// variable lines contain a name, and equal sign and then a value
// variable lines can also only contain a name (the implicit value is a boolean true)
// variable name is alphanumeric (ASCII) with -
// variable name starts with an alphabetic character
// variable name is case insensitive
const VARIABLE_LINE_REGEX: Regex = Regex::new(r#"^([A-Za-z][A-Za-z-]*)(?: *= *(.*))?$"#).unpack();
const VARIABLE_NAME_REGEX: Regex = Regex::new(r#"^[A-Za-z][A-Za-z-]*$"#).unpack();

const VARIABLE_VALUE_COMMENT_REGEX: Regex = Regex::new(r#"^(.*?)( *[#;].*)$"#).unpack();

fn extract_section_line(line: &str) -> Option<(&str, &str)> {
    let matches = SECTION_LINE_REGEX.captures(line);

    return matches.map(|cap| {
        match (cap.get(1), cap.get(2)) {
            (Some(cap1), Some(cap2)) => (cap1.as_str(), cap2.as_str()),
            _ => { None }
        }
    });
}

fn extract_variable_line(line: &str) -> Option<(&str, &str)> {
    let matches = SECTION_LINE_REGEX.captures(line);

    return matches.map(|cap|
        (
            cap.get(1).map(|m| m.as_str()),
            cap.get(2).map(|m| m.as_str()).unwrap_or("true")
        )
    )
        .map(|(name_opt, raw_value_opt)| {
            match (name_opt, raw_value_opt) {
                (Some(name), raw_value) => (name, raw_value),
                _ => { None }
            }
        })
        .map(|(name, raw_value)| {
            let value_without_comments = remove_comments(raw_value);
            let value_without_quotes = remove_quotes(value_without_comments);
            (name, value_without_quotes)
        });
}

// removeComments
fn remove_comments(raw_value: &str) -> &str {
    let comment_matches = VARIABLE_VALUE_COMMENT_REGEX.captures(raw_value);

    return match comment_matches
        .map(|caps| {
            (
                caps.get(1).map(|m| m.as_str()),
                caps.get(2).map(|m| m.as_str())
            )
        })
        .map(|(value_without_comment_opt, comment_opt)| {
            match (name_opt, raw_value_opt) {
                (Some(value_without_comment), Some(comment)) => (value_without_comment, comment),
                _ => { None }
            }
        })
        .map(|(value_without_comment, comment)| {
            if has_odd_number_of_quotes(value_without_comment) && has_odd_number_of_quotes(comment) {
                return format!("{}{}", value_without_comment, comment).as_str();
            }
            return value_without_comment;
        }) {
        Some(val) => val,
        _ => { raw_value }
    };
}

fn has_odd_number_of_quotes(text: &str) -> bool {
    let quote_regex = Regex::new(r#"(?g)(?:^|[^\\])""#);
    let number_of_quotes = quote_regex.captures_len(text);
    return number_of_quotes % 2 != 0;
}

fn remove_quotes(text: &str) -> &str {
    let mut new_text = "";
    for (idx, c) in text.split("").enumerate() {
        let is_quote = c == r#""""# && text[idx - 1] != '\\';
        let is_escape_for_quote = c == '\\' && text[idx + 1] == '"';
        if !is_quote && !is_escape_for_quote {
            new_text = new_text + c;
        }
    }

    return new_text;
}


// Note: there are a LOT of edge cases that aren't covered (e.g. keys in sections that also
// have subsections, [include] directives, etc.
impl Parse {
    pub(crate) fn from(&self, text: String) {
        let mut section: &str;
        let mut subsection: &str;

        let parsedConfig = text.split("\n").map(|line| {
            let mut name: &str = "";
            let mut value: &str = "";

            let trimmed_line = line.trim();
            let is_section = extract_section_line(trimmed_line);
            match is_section {
                Some(capSection) => {
                    section = capSection[0];
                    subsection = capSection[1];
                }
                None => {
                    (name, value) = extract_variable_line(trimmed_line).unwrap();
                }
            }
        });
    }
}