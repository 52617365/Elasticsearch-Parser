mod format;
mod read;
use crate::format::format::format_pattern;
use crate::format::format::parse_file;
use crate::read::read::list_directories;
fn main() {
    //    let pattern = format_pattern("[user pass email]");
    let dirs = list_directories(r"/home/floppa/dev/rust/elastic_parser/src/example_dir");
    println!("{:?}", dirs);

    let pattern = format_pattern("[user pass mail]");
    if !pattern.is_empty() {
        let combo = "hellobro:hellopass:hellomail";
        let jsonthing = parse_file(pattern, combo);
        println!("{:?}", jsonthing);
    } else {
        println!("{}", "Error with regex");
    }
}
