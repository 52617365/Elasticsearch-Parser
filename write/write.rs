use std::fs::{File, self};
use std::path::PathBuf;
use std::io::prelude::*;

pub fn write_json_strings_to_file(file_name : &PathBuf, json_strings : Vec<String>) -> std::io::Result<()>{
    let mut f = File::create(file_name)?;

    for json_string in json_strings {
        writeln!(f, "{}", json_string)?;
    }
    Ok(())
}

pub fn write_logs_to_file(message : &str, file_handle : &mut File) -> std::io::Result<()>{
    writeln!(file_handle, "{}", message)?;
    Ok(())
}
