mod format;
mod read;
mod write;
use std::io;
use crate::read::read::list_directories;
use crate::read::read::iterate_directory_files_and_convert;

fn main() -> io::Result<()>{
    let relative_path = "../data/unparsed_data";

    let files = list_directories(&relative_path);
    let _x = iterate_directory_files_and_convert(files);
    Ok(())
}
