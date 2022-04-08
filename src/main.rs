mod format;
mod read;
mod write;
use std::io;
use crate::format::format::format_pattern;
use crate::format::format::line_to_json;
use crate::read::read::list_directories;
use crate::read::read::iterate_file_lines;

fn main() -> io::Result<()>{
    let dirs = list_directories("/home/floppa/dev/rust/elastic_parser/tests/example_dir");



    Ok(())
}
