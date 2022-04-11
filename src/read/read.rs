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
pub fn list_directories(path: &str) -> Option<Vec<PathBuf>> {
    if !Path::new(path).exists() {
        return None;
    }

    let complete_path = format!("{}{}", path, "/**/*.txt");

    let mut directories: Vec<PathBuf> = Vec::with_capacity(5);
    for entry in glob(&complete_path).expect("Error listing text files") {
        match entry {
            Ok(path) => directories.push(path),
            Err(_) => (),
        }
    }
    Some(directories)
}

pub fn start_iterating_files(files: Vec<PathBuf>) -> io::Result<()> {
    for file in files.iter() {
        let serialized_strings = match iterate_file_lines(file) {
            Ok(result) => result,
            Err(_) => continue,
        };
        println!("{}", file.display());
        write_json_strings_to_file(file, serialized_strings)?;
    }
    Ok(())
}

pub fn iterate_file_lines(file: &Path) -> io::Result<Vec<String>> {
    let lines = read_file_into_lines(file)?;

    let file_format = format_pattern(&lines[0]);

    let (line_delimiter, line_format) = get_delimiter_and_format_from_file(file_format); // First line contains the format and delimiter so we run it through regex.

    // We want to store file name in the data so we get it here.
    let file_name = Path::new(&file)
        .file_name()
        .expect("Error getting file name")
        .to_string_lossy();

    let serialized_lines = lines_to_json(line_format, &lines, &line_delimiter, &file_name)?;
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
