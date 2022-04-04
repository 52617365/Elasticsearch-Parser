use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub x: HashMap<String, String>,
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
pub fn parse_file(keys: &str, data: &str) -> Result<String> {
    let parsed_keys: Vec<&str> = keys.split_whitespace().collect();
    let parsed_line: Vec<&str> = data.split(":").collect();
    let mut values = Data {
        x: HashMap::with_capacity(parsed_line.len()),
    };
    // Parsed keys and parsed line should be the same length so we iterate over container with parsed keys length
    for key in 0..parsed_keys.len() {
        values
            .x
            .insert(parsed_keys[key].to_owned(), parsed_line[key].to_owned());
    }

    // Using serde json here to turn the hashmap into a json string.
    let serialized = serde_json::to_string(&values.x)?;
    Ok(serialized)
}
