use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::BTreeMap;
use std::iter::zip;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub x: BTreeMap<String, String>,
}

// Reads first line containing format.
// We use regex because we don't want to run into situations where the program mistakes a random line as a format when it's not.
// If file does not contain a format we return an empty string.
pub fn format_pattern(format: &str) -> &str {
    let re = Regex::new(r"\[([^]]+)\]").unwrap();
    match re.captures(format) {
        Some(caps) => caps.get(1).unwrap().as_str(),
        None => "",
    }
}

// Gets called for each line in a file and returns a JSON String if everything goes to plan.
pub fn lines_to_json(
    line_format: &str,
    lines: &Vec<String>,
    line_delimiter: &str,
    file_name: &str,
) -> Result<Vec<String>> {
    let mut serialized_lines: Vec<String> = Vec::with_capacity(lines.len());

    for line in lines[1..].iter() {
        let mut values = Data { x: BTreeMap::new() };

        let parsed_line: Vec<&str> = line.split(line_delimiter).collect(); // Splits the line with the specified delimiter.

        let format_container: Vec<&str> = line_format.split_whitespace().collect(); // Splits the format into a container.
                                                                                    // Add line into json string container if serializing did not fail, else do nothing.

        for index in 0..format_container.len() {
            values.x.insert(
                format_container[index].to_string(),
                parsed_line[index].to_owned(),
            );
            values.x.insert("filename".to_owned(), file_name.to_owned());
        }

        let serialized_line = serde_json::to_string(&values.x)?;
        serialized_lines.push(serialized_line);
    }
    // Parsed keys and parsed line should be the same length so we iterate over container with parsed keys length

    Ok(serialized_lines)
}
