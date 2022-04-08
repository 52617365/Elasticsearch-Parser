use crate::read::read::list_directories;
use crate::read::read::iterate_directory_files;

pub fn run() {
    let files = list_directories("/home/floppa/dev/rust/elastic_parser/example_dir");
    let _ = iterate_directory_files(files);
}
