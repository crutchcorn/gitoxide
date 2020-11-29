use regex::Regex;

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


fn extract_section_line(line: &str) -> Option<Vec<&str>> {
    let matches = SECTION_LINE_REGEX.captures_iter(line);

    let vec: Vec<&str> = matches.collect();

    if vec.len() == 0 {return None;}

    Some(Vec::from(&vec[1..vec.len()]))
}


fn extractVariableLine(line: &str) -> Option<Vec<&str>> {
    let matches = SECTION_LINE_REGEX.captures(line)

    match matches {
        Some(cap) => {
            let name = matches.get(1).map(|mx| mx.as_str()).unwrap_or("");
            let rawValue = matches.get(2).map(|mx| mx.as_str()).unwrap_or("true");
            // const valueWithoutComments = removeComments(rawValue)
            // const valueWithoutQuotes = removeQuotes(valueWithoutComments)
            // return [name, valueWithoutQuotes]
            Some(vec![""])
        },
        None => { }
    }
}

impl Parse {
    pub(crate) fn from(&self, text: String) {
        let mut section: &str;
        let mut subsection: &str;

        let parsedConfig = text.split("\n").map(|line| {
            let mut name = None;
            let mut value = None;

            let trimmed_line = line.trim();
            let is_section = extract_section_line(trimmed_line);
            match is_section {
                Some(capSection) => {
                    section = capSection[0];
                    subsection = capSection[1];
                }
                None => {

                }
            }
        });
    }
}