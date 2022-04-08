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
            Err(_) => (),
        }
    }
    directories
}

pub fn iterate_directory_files(files : Vec<PathBuf>) -> io::Result<()> {
    for file in files.iter() {
    let filepath_str = file.to_string_lossy();
      let serialized_lines = iterate_file_lines(&filepath_str);
      // Lines are json strings at this point.
      let serialized_lines = match serialized_lines {
          Ok(lines) => lines,
          Err(_) => continue,
      };

      // If line iterator failed to parse file lines into json, just move to the next file.
      if serialized_lines.len() == 0 {
          continue;
      }

      let write = write_json_strings_to_file(file, serialized_lines);
      let _ = match write {
            // TODO: Write these into a log file.
              Ok(_) => println!("Successfully wrote to {:?}", file),
              Err(_) => println!("Error writing to {:?}", file),
        };
     }
    Ok(())
}

pub fn iterate_file_lines(file : &str) -> io::Result<Vec<String>> {
       let lines = read_file_into_lines(file)?; // Get lines from a file and if it fails to do so, skip to the next file.

       let file_format = format_pattern(&lines[0]);

       // If can't determine file format, dont do anything (String is empty if regex failed to get a match.)
       if file_format.is_empty() {
           return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error determining format"));
       }

       let (line_delimiter, line_format) = get_delimiter_and_format_from_file(file_format); // First line contains the format and delimiter so we run it through regex.

       // Some sort of format is guaranteed here.
       let mut serialized_lines : Vec<String> = Vec::with_capacity(lines.len());
       let file_name = Path::new(&file).file_name().unwrap().to_string_lossy();
       for line in lines[1..].iter() {
            let serialized_line = line_to_json(line_format, line, line_delimiter, &file_name);

            // Add line into json string container if serializing did not fail, else do nothing.
            let _ = match serialized_line {
                Ok(line) => serialized_lines.push(line),
                Err(_) => (),
           };
      }
      Ok(serialized_lines)
}

fn read_file_into_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_delimiter_and_format_from_file(parsed_line: &str) -> (&str, &str) {
           let file_delimiter = &parsed_line[0..1]; // After regex the first index of the string is the delimiter.
           let file_format = &parsed_line[1..]; // The rest is the actual format.
           (file_delimiter, file_format)
}
