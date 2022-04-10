use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;

pub fn write_json_strings_to_file(file_name : &PathBuf, json_strings : Vec<String>) -> std::io::Result<()>{
    let path = Path::new("../data/parsed_data/").join(file_name);
    println!("{}", path.display());
    let mut f = File::create(path)?;

    for json_string in json_strings {
        writeln!(f, "{}", json_string)?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn write_logs_to_file(message : &str, file_handle : &mut File) -> std::io::Result<()>{
    writeln!(file_handle, "{}", message)?;
    Ok(())
}