use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // https://git-scm.com/docs/git-config#_syntax

    // section starts with [ and ends with ]
    // section is alphanumeric (ASCII) with - and .
    // section is case insensitive
    // subsection is optional
    // subsection is specified after section and one or more spaces
    // subsection is specified between double quotes
    static ref SECTION_LINE_REGEX: Result<regex::Regex, regex::Error> = Regex::new(r#"^\[([A-Za-z0-9-.]+)(?: "(.*)")?\]$"#);
    static ref SECTION_REGEX: Result<regex::Regex, regex::Error> = Regex::new(r#"^[A-Za-z0-9-.]+$"#);

    // variable lines contain a name, and equal sign and then a value
    // variable lines can also only contain a name (the implicit value is a boolean true)
    // variable name is alphanumeric (ASCII) with -
    // variable name starts with an alphabetic character
    // variable name is case insensitive
    static ref VARIABLE_LINE_REGEX: Result<regex::Regex, regex::Error> = Regex::new(r#"^([A-Za-z][A-Za-z-]*)(?: *= *(.*))?$"#);
    static ref VARIABLE_NAME_REGEX: Result<regex::Regex, regex::Error> = Regex::new(r#"^[A-Za-z][A-Za-z-]*$"#);

    static ref VARIABLE_VALUE_COMMENT_REGEX: Result<regex::Regex, regex::Error> = Regex::new(r#"^(.*?)( *[#;].*)$"#);
}

fn extract_section_line(line: &str) -> Option<(&str, &str)> {
    let matches = SECTION_LINE_REGEX.as_ref().unwrap().captures(line);

    matches.and_then(|cap| {
        match (cap.get(1), cap.get(2)) {
            (Some(cap1), Some(cap2)) => Some((cap1.as_str(), cap2.as_str())),
            _ => { None }
        }
    })
}

fn extract_variable_line(line: &str) -> Option<(String, String)> {
    let matches = VARIABLE_LINE_REGEX.as_ref().unwrap().captures(line);

    return matches.map(|cap|
        (
            cap.get(1).map(|m| m.as_str()),
            cap.get(2).map(|m| m.as_str()).unwrap_or("true")
        )
    )
        .and_then(|(name_opt, raw_value_opt)| {
            match (name_opt, raw_value_opt) {
                (Some(name), raw_value) => Some((name, raw_value)),
                _ => { None }
            }
        })
        .map(|(name, raw_value)| {
            let value_without_comments = remove_comments(raw_value);
            let value_without_quotes = remove_quotes(&value_without_comments);
            (name.to_string(), value_without_quotes)
        });
}

// removeComments
fn remove_comments(raw_value: &str) -> String {
    let comment_matches = VARIABLE_VALUE_COMMENT_REGEX.as_ref().unwrap().captures(raw_value);

    return match comment_matches
        .map(|caps| {
            (
                caps.get(1).map(|m| m.as_str()),
                caps.get(2).map(|m| m.as_str())
            )
        })
        .and_then(|(value_without_comment_opt, comment_opt)| {
            match (value_without_comment_opt, comment_opt) {
                (Some(value_without_comment), Some(comment)) => Some((value_without_comment, comment)),
                _ => { None }
            }
        })
        .map(|(value_without_comment, comment)| {
            if has_odd_number_of_quotes(value_without_comment) && has_odd_number_of_quotes(comment) {
                return format!("{}{}", value_without_comment, comment);
            }
            return value_without_comment.to_string();
        }) {
        Some(val) => val,
        _ => { raw_value.to_string() }
    };
}

fn has_odd_number_of_quotes(text: &str) -> bool {
    let quote_regex = Regex::new(r#"(?g)(?:^|[^\\])""#).unwrap();
    let number_of_quotes = quote_regex.captures(text);
    return number_of_quotes.unwrap().len() % 2 != 0;
}

fn remove_quotes(text: &String) -> String {
    let mut new_text = "".to_owned();
    for (idx, c) in text.split("").enumerate() {
        let is_quote = c == r#""""# && text.chars().nth(idx - 1).unwrap_or(' ') != '\\';
        let is_escape_for_quote = c == "\\" && text.chars().nth(idx + 1).unwrap_or(' ') == '"';
        if !is_quote && !is_escape_for_quote {
            new_text.push_str(c);
        }
    }

    return new_text;
}

fn get_path(section: &str, subsection: &str, name: &String) -> String {
    let filtered: Vec<String> = vec![section.to_lowercase(), subsection.to_string(), name.to_lowercase()].into_iter().filter(|string| !string.is_empty()).collect();
    return filtered.join(".");
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParsedConfig {
    line: String,
    is_section: bool,
    section: String,
    subsection: String,
    name: String,
    value: String,
    path: String
}

// Note: there are a LOT of edge cases that aren't covered (e.g. keys in sections that also
// have subsections, [include] directives, etc.
pub fn parse_config(text: String) -> Vec<ParsedConfig> {
    let mut section: &str = "";
    let mut subsection: &str = "";

    let parsed_config = text.split("\n").map(|line| {
        let mut name: String = "".to_string();
        let mut value: String = "".to_string();

        let trimmed_line = line.trim();
        let extracted_section = extract_section_line(trimmed_line);
        let mut is_section = false;
        match extracted_section {
            Some((section_temp, subsection_temp)) => {
                section = section_temp;
                subsection = subsection_temp;
                is_section = true;
            }
            None => {
                let (name_temp, value_temp) = extract_variable_line(trimmed_line)
                    .unwrap_or((name, value));
                name = name_temp;
                value = value_temp;
            }
        }

        let path = get_path(section, subsection, &name);
        ParsedConfig {
            line: line.to_string(),
            is_section,
            section: section.to_string(),
            subsection: subsection.to_string(),
            name: name.to_string(),
            value: value.to_string(),
            path: path.to_string()
        }
    }).collect();

    return parsed_config;
}
