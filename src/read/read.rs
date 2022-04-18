use crate::format::format::lines_to_json;
use crate::write::write::write_json_strings_to_file;
use glob::glob;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

// Lists all text files inside of the current directory and ALL child directories in the specified path.
pub fn list_files(path: &str) -> Vec<String> {
    let mut unparsed_directories: Vec<String> = Vec::with_capacity(50);
    if Path::new(&path).exists() {
        let complete_path = format!("{}{}", path, "/**/*.txt");
        for entry in glob(&complete_path).expect("Error listing text files") {
            match entry {
                Ok(path) => unparsed_directories.push(path.to_string_lossy().to_string()),
                Err(_) => (),
            }
        }
    }
    unparsed_directories
}
// Function checks all parsed and unparsed files and extracts the ones that are not yet parsed.
pub fn list_unparsed_files(unparsed_path: &str, parsed_path: &str) -> Option<Vec<String>> {
    let unparsed_files = list_files(unparsed_path);
    let parsed_files = list_files(parsed_path);

    let mut extracted_files = Vec::with_capacity(unparsed_files.len());

    for unparsed_file in unparsed_files.iter() {
        let filename = Path::new(unparsed_file).file_name()?;
        if !parsed_files.iter().any(|parsed_file| {
            parsed_file.contains(filename.to_str().expect("Error parsing filename"))
        }) {
            extracted_files.push(unparsed_file.to_string());
        }
    }

    // Check if there are any files left to parse
    return if extracted_files.is_empty() {
        None
    } else {
        Some(extracted_files)
    };
}

pub fn start_iterating_files(files: Vec<String>) -> io::Result<()> {
    for file in files.iter() {
        let file_path = PathBuf::from(file);
        let serialized_strings = match iterate_file_lines(&file_path) {
            Ok(result) => result,
            Err(_) => continue,
        };
        write_json_strings_to_file(&file_path, serialized_strings)?;
    }
    Ok(())
}

pub fn iterate_file_lines(file: &Path) -> Result<Vec<String>, &str> {
    let lines: Vec<String> = match read_file_into_lines(file) {
        Ok(lines) => lines,
        Err(_) => return Err("Error getting lines from file."),
    };

    // Lines is an empty vector if file was empty.
    // We return because if we don't have anything to work with it's useless to continue.
    if lines.is_empty() {
        return Err("Error processing file, it's empty.");
    }

    let file_format: &str = &lines[0];

    // Regex returns an empty string if it didn't find a match.
    if file_format.is_empty() {
        return Err("Error getting format, it's not determined.");
    }

    let (line_delimiter, line_format) = get_delimiter_and_format_from_file(file_format); // First line contains the format and delimiter so we run it through regex.

    // We want to store file name in the data so we get it here.
    let file_name = Path::new(&file)
        .file_name()
        .expect("Error getting file name")
        .to_string_lossy();

    let serialized_lines = match lines_to_json(line_format, &lines, &line_delimiter, &file_name) {
        Ok(json_lines) => json_lines,
        Err(_) => return Err("Error converting lines to json."),
    };
    Ok(serialized_lines)
}

pub fn read_file_into_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_delimiter_and_format_from_file(parsed_line: &str) -> (&str, &str) {
    let file_delimiter = &parsed_line[0..1]; // After regex the first index of the string is the delimiter.
    let file_format = &parsed_line[1..]; // The rest is the actual format.
    (file_delimiter, file_format)
}
