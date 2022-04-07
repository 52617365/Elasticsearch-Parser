use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

pub fn write_json_strings_to_file(file_name : &PathBuf, json_strings : Vec<String>) -> std::io::Result<()>{
    let mut f = File::create(file_name)?;

    for json_string in json_strings {
        writeln!(f, "{}", json_string)?;
    }
    Ok(())
}
