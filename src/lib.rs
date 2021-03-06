mod format;
mod read;
mod write;
#[cfg(test)]
mod tests {
    use crate::read::read;
    use std::{
        io::{self, BufRead, BufReader},
        path::{Path, PathBuf},
    };

    #[test]
    fn test_extract_unparsed_files() {
        use crate::read::read::list_unparsed_files;
        let unparsed_files = "tests/unparsed_files";
        let parsed_files = "tests/parsed_files";

        assert!(Path::new(unparsed_files).is_dir());
        assert!(Path::new(parsed_files).is_dir());

        let expected_file = "tests/unparsed_files/example2.txt";

        let expected_unparsed_files = vec![expected_file];

        let paths = list_unparsed_files(unparsed_files, parsed_files)
            .expect("There were no unparsed files left.");

        assert_eq!(expected_unparsed_files, paths);
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
        let example1 = "tests/example_dir/exampledata/exampledata.txt";
        let example2 = "tests/example_dir/exampledata2/exampledata2.txt";
        let example3 = "tests/example_dir/example2/example2.txt";

        let expected_paths = vec![example3, example1, example2];

        let path = Path::new("./tests/example_dir");
        //      let path_to_unparsed_files = Path::new("./tests/unparsed_dir");
        //     let path_to_parsed_files = Path::new("./tests/parsed_dir");
        assert!(path.exists());

        //        let path_to_unparsed_files: String = String::from(path_to_unparsed_files.to_string_lossy());
        //       let path_to_parsed_files: String = String::from(path_to_parsed_files.to_string_lossy());
        let path: String = String::from(path.to_string_lossy());

        let list_paths = read::list_files(&path);

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

        if &json_string_one[0] == expected_json_string_one
            && &json_string_two[0] == expected_json_string_two
        {
            Ok(())
        } else {
            Err(String::from("There was an error with serializing lines"))
        }
    }

    #[test]
    fn test_file_reading() -> io::Result<()> {
        let file = Path::new("./tests/example_dir/exampledata/exampledata.txt");
        let lines = read::read_file_into_lines(file)?; // Get lines from a file and if it fails to do so, skip to the next file.

        let expected_file_contents = vec![
            ":latitude longitude temperature",
            "20.2:12.3:20",
            "27.2:12.3:20",
            "26.2:12.3:20",
            "24.2:12.3:20",
            "21.2:12.3:20",
        ];

        println!("{:?}", expected_file_contents);
        println!("{:?}", lines);
        assert_eq!(expected_file_contents, lines);
        Ok(())
    }
}
