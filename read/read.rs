use glob::glob;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    };
use crate::format::format::format_pattern;
use crate::format::format::line_to_json;
use crate::write::write::write_json_strings_to_file;

// Lists all text files inside of the current directory and ALL child directories in the specified path.
// TODO: Support other file types too after testing.
pub fn list_directories(path: &str) -> Vec<PathBuf> {
    let path = format!("{}{}", path, "/**/*.txt");

    let mut directories: Vec<PathBuf> = Vec::with_capacity(3000);
    for entry in glob(&path).expect("Error listing text files") {
        match entry {
            Ok(path) => directories.push(path),
            Err(e) => println!("Error adding {} to paths", e),
        }
    }
    directories
}


pub fn iterate_files() -> io::Result<()> {
    let files = list_directories("/home/floppa/dev/rust/elastic_parser/example_dir");

    for file in files.iter() {
      let serialized_lines = iterate_file_lines(file); // Lines are json strings at this point.
      let serialized_lines = match serialized_lines {
          Ok(lines) => lines,
          Err(_) => continue,
      };
      if serialized_lines.len() != 0  {
        let write =   write_json_strings_to_file(file, serialized_lines);
        let _ = match write {
                Ok(_) => println!("Successfully wrote to {:?}", file),
                Err(_) => println!("Error writing to {:?}", file),
        };
      }
     }

    Ok(())
}


pub fn iterate_file_lines(file : &PathBuf) -> io::Result<Vec<String>> {
       let lines = get_lines_from_file(file)?; // Get lines from a file and if it fails to do so, skip to the next file.

       let parsed_file_format = format_pattern(&lines[0]); // First line contains the format and delimiter so we run it through regex.
       let (line_delimiter, line_format) = get_delimiter_and_format_from_file(parsed_file_format);

       let mut serialized_lines : Vec<String> = Vec::with_capacity(lines.len());

       for line in lines[1..].iter() {
            let serialized_line = line_to_json(line_format, line, line_delimiter);

            // Add line into json string container if serializing did not fail, else do nothing.
            let _ = match serialized_line {
                Ok(line) => serialized_lines.push(line),
                Err(_) => (),
           };
      }
      Ok(serialized_lines)
}

pub fn get_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn get_delimiter_and_format_from_file(parsed_line: &str) -> (&str, &str) {
           let file_delimiter = &parsed_line[0..1]; // After regex the first index of the string is the delimiter.
           let file_format = &parsed_line[1..]; // The rest is the actual format.
           (file_delimiter, file_format)
}
