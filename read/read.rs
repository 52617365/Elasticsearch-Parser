use glob::glob;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

pub fn list_directories(path: &str) -> Vec<PathBuf> {
    let path = format!("{}{}", path, "/**/*.txt");

    let mut directories: Vec<PathBuf> = Vec::new();
    for entry in glob(&path).expect("Error listing text files") {
        match entry {
            Ok(path) => directories.push(path),
            Err(e) => println!("{:?}", e),
        }
    }
    directories
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
