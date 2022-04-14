mod format;
mod read;
mod write;
use crate::read::read::list_unparsed_files;
use crate::read::read::start_iterating_files;
use std::env;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "FULL");

    let path_to_unparsed_files = Path::new("../data/unparsed_data/");
    let path_to_parsed_files = Path::new("../data/parsed_data/");

    let files = match list_unparsed_files(
        &path_to_unparsed_files.to_string_lossy(),
        &path_to_parsed_files.to_string_lossy(),
    ) {
        Some(files) => files,
        None => panic!("No files found."),
    };
    let _ = start_iterating_files(files);
    Ok(())
}
