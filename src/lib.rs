mod format;
mod read;
mod write;
#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::read::read;
    use glob::glob;

    #[test]
    fn test_regex() {
        use crate::format::format;

        let expected_unparsed_string = "[:Latitude Longitude Coordinates]";

        let regex_string = format::format_pattern(expected_unparsed_string);

        let expected_string = ":Latitude Longitude Coordinates";

        assert_eq!(regex_string, expected_string);
    }

    #[test]
    fn test_files_exist() {
        let example1 = Path::new("tests/example_dir/exampledata/exampledata.txt");
        let example2 = Path::new("tests/example_dir/exampledata2/exampledata2.txt");
        let example3 = Path::new("tests/example_dir/example2/example2.txt");

        assert!(example1.exists());
        assert!(example2.exists());
        assert!(example3.exists());
    }

    #[test]
    fn test_directory_iteration() {
        let example1 = Path::new("tests/example_dir/exampledata/exampledata.txt");
        let example2 = Path::new("tests/example_dir/exampledata2/exampledata2.txt");
        let example3 = Path::new("tests/example_dir/example2/example2.txt");

        let expected_paths = vec![example3, example1, example2];

        let path_to_files = Path::new("./tests/example_dir");
        assert!(path_to_files.exists());
        let path: String =String::from(path_to_files.to_string_lossy());

        let list_paths = read::list_directories(&path);

        println!("got {:?}", list_paths);
        println!("expected {:?}", expected_paths);

        assert_eq!(expected_paths, list_paths);
    }

    #[test]
    fn test_file_iteration() -> Result<(), String> {
        // First file path contains a ":" delimited dataset
        let file_path = Path::new("./tests/example_dir/exampledata/exampledata.txt");
        // Second file path contains a ";" delimited dataset
        let file_path2 = Path::new("./tests/example_dir/exampledata2/exampledata2.txt");
        assert!(Path::new(&file_path).is_file());
        assert!(Path::new(&file_path2).is_file());

        let expected_json_string_one = r#"{"filename":"exampledata.txt","latitude":"20.2","longitude":"12.3","temperature":"20"}"#;
        let expected_json_string_two = r#"{"filename":"exampledata2.txt","latitude":"20.2","longitude":"12.3","temperature":"20"}"#;

        let json_string_one = read::iterate_file_lines(&file_path).unwrap();
        let json_string_two = read::iterate_file_lines(&file_path2).unwrap();

        println!("expected {}", expected_json_string_one);
        println!("got {:?}", json_string_one);

        if &json_string_one[0] == expected_json_string_one && &json_string_two[0] == expected_json_string_two {
            Ok(())
        }
        else {
            Err(String::from("There was an error with serializing lines"))
        }
    }
}
