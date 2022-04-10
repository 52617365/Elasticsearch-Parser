mod format;
mod read;
mod write;
use std::io;
use std::path::Path;
use crate::read::read::list_directories;
use crate::read::read::read_file_into_lines;
use crate::read::read::start_iterating_files;

fn main() -> io::Result<()>{
    //cultcraft.me_9.4k.txt
    let path_to_files = Path::new("../data/unparsed_data/exampledata.txt");

    let files = match list_directories(&path_to_files.to_string_lossy()) {
        Some(files) => files,
        None => panic!("No files found."),
    };

    let _ = start_iterating_files(files);
    Ok(())
}
