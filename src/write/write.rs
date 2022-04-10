use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn write_json_strings_to_file(file_name : &Path, json_strings : Vec<String>) -> std::io::Result<()>{
    if json_strings.len() == 0 {
           return Err(std::io::Error::new(std::io::ErrorKind::Other, "Json strings were empty"));
    }
    let file_name = file_name.file_name().expect("Error getting file name").to_string_lossy();

    let path = Path::new("../data/parsed_data/").join(file_name.to_string());
    println!("{}", path.display());

    let mut f = File::create(path)?;

    for json_string in json_strings.iter() {
        writeln!(f, "{}", json_string)?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn write_logs_to_file(message : &str, file_handle : &mut File) -> std::io::Result<()>{

    writeln!(file_handle, "{}", message)?;
    Ok(())
}

