use crate::format::format::format_pattern;
use crate::format::format::lines_to_json;
use crate::write::write::write_json_strings_to_file;
use glob::glob;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

// Lists all text files inside of the current directory and ALL child directories in the specified path.
// TODO: Support other file types too after testing.

pub fn list_files(path: &str) -> Vec<PathBuf> {
    let mut unparsed_directories: Vec<PathBuf> = Vec::with_capacity(50);
    if Path::new(path).exists() {
        let complete_path = format!("{}{}", path, "/**/*.txt");
        for entry in glob(&complete_path).expect("Error listing text files") {
            match entry {
                Ok(path) => unparsed_directories.push(path),
                Err(_) => (),
            }
        }
    }
    unparsed_directories
}
pub fn list_unparsed_files(unparsed_path: &str, parsed_path: &str) -> Option<Vec<PathBuf>> {
    let unparsed_files = list_files(unparsed_path);

    let parsed_files = list_files(parsed_path);

    let mut directories: Vec<PathBuf> = Vec::with_capacity(unparsed_path.len());

    // Get the files that are not already parsed.
    for file in unparsed_files.iter() {
        if !parsed_files.contains(&file) {
            directories.push(file.to_path_buf());
        }
    }
    // Check if there are any files left to parse
    if directories.len() == 0 {
        return None;
    } else {
        return Some(directories);
    }
}

pub fn start_iterating_files(files: Vec<PathBuf>) -> io::Result<()> {
    for file in files.iter() {
        let serialized_strings = match iterate_file_lines(file) {
            Ok(result) => result,
            Err(_) => continue,
        };
        write_json_strings_to_file(file, serialized_strings)?;
    }
    Ok(())
}

pub fn iterate_file_lines(file: &Path) -> io::Result<Vec<String>> {
    let lines: Vec<String> = read_file_into_lines(file)?;

    let file_format = format_pattern(&lines[0]);

    if !file_format.is_empty() {
        let (line_delimiter, line_format) = get_delimiter_and_format_from_file(file_format); // First line contains the format and delimiter so we run it through regex.

        // We want to store file name in the data so we get it here.
        let file_name = Path::new(&file)
            .file_name()
            .expect("Error getting file name")
            .to_string_lossy();

        let serialized_lines = lines_to_json(line_format, &lines, &line_delimiter, &file_name)?;

        Ok(serialized_lines)
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error reading file",
        ));
    }
}

pub fn read_file_into_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_delimiter_and_format_from_file(parsed_line: &str) -> (&str, &str) {
    let file_delimiter = &parsed_line[0..1]; // After regex the first index of the string is the delimiter.
    let file_format = &parsed_line[1..]; // The rest is the actual format.
    (file_delimiter, file_format)
}
