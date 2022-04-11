mod format;
mod read;
mod write;
use crate::read::read::list_directories;
use crate::read::read::start_iterating_files;
use std::env;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    //cultcraft.me_9.4k.txt
    let path_to_files = Path::new("../data/unparsed_data/");

    println!("{:?}", path_to_files);
    let files = match list_directories(&path_to_files.to_string_lossy()) {
        Some(files) => files,
        None => panic!("No files found."),
    };
    let _ = start_iterating_files(files);
    Ok(())
}
