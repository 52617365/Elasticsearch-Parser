mod format;
mod read;
mod write;
use std::path::PathBuf;
use std::io;
use crate::format::format::format_pattern;
use crate::format::format::line_to_json;
use crate::read::read::list_directories;
use crate::read::read::iterate_file_lines;

fn main() -> io::Result<()>{
    Ok(())
}
