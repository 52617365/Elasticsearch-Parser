mod format;
mod read;
mod write;
use std::io;
use std::path::Path;
use crate::read::read::list_directories;
use crate::read::read::iterate_directory_files;

fn main() -> io::Result<()>{
    let relative_path = Path::new("./data/unparsed_data").to_string_lossy();

    let files = list_directories(&relative_path);
    println!("{:?}", files);
    let _x = iterate_directory_files(files);
    Ok(())
}
