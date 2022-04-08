mod format;
mod read;
mod write;
mod tests {
    use std::{path::Path, io};
    use std::path::{PathBuf, self};
    use crate::read::read;

    #[cfg(test)]
    #[test]
    fn test_regex() {
        use crate::format::format;

        let expected_unparsed_string = "[:Latitude Longitude Coordinates]";

        let regex_string = format::format_pattern(expected_unparsed_string);

        let expected_string = ":Latitude Longitude Coordinates";

        assert_eq!(regex_string, expected_string);
    }

    #[test]
    fn test_directory_iteration() {
        let example1 = PathBuf::from("/home/floppa/dev/rust/elastic_parser/tests/example_dir/exampledata.txt");
        let example2 = PathBuf::from("/home/floppa/dev/rust/elastic_parser/tests/example_dir/example2.txt");
        let example3 = PathBuf::from("/home/floppa/dev/rust/elastic_parser/tests/example_dir/exampledata2.txt");

        let expected_paths = vec![example2, example1, example3];

        let path_to_files = PathBuf::from("/home/floppa/dev/rust/elastic_parser/tests/example_dir");
        assert!(path_to_files.exists());

        let list_paths = read::list_directories("/home/floppa/dev/rust/elastic_parser/tests/example_dir");

        assert_eq!(expected_paths, list_paths);
    }

    #[test]
    fn test_file_iteration() -> Result<(), String> {
        // First file path contains a ":" delimited dataset
        let file_path = PathBuf::from("/home/floppa/dev/rust/elastic_parser/tests/example_dir/exampledata.txt");
        // Second file path contains a ";" delimited dataset
        let file_path2 = PathBuf::from("/home/floppa/dev/rust/elastic_parser/tests/example_dir/exampledata2.txt");
        assert!(Path::new(&file_path).is_file());
        assert!(Path::new(&file_path2).is_file());

        let expected_json_string_one = r#"{"filename":"exampledata.txt","latitude":"20.2","longitude":"12.3","temperature":"20"}"#;
        let expected_json_string_two = r#"{"filename":"exampledata2.txt","latitude":"20.2","longitude":"12.3","temperature":"20"}"#;

        let filename_one_to_string = file_path.to_string_lossy();
        let filename_two_to_string = file_path2.to_string_lossy();

        let json_string_one = read::iterate_file_lines(&filename_one_to_string).unwrap();
        let json_string_two = read::iterate_file_lines(&filename_two_to_string).unwrap();

        println!("{}", json_string_one[0]);
        println!();
        println!("{}", json_string_two[0]);

        if &json_string_one[0] == expected_json_string_one  && &json_string_two[0] == expected_json_string_two {
            Ok(())
        }
        else {
            Err(String::from("There was an error with serializing lines"))
        }
    }

}
