use minigrep::{Cli, run};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_cli_file_path() {
        let cli = Cli::from_args(vec![
            String::from("filename"),
            String::from("file_path"),
            String::from("query")
        ]);
        assert_eq!(*cli.file_path(), String::from("file_path"));
    }

    #[test]
    fn test_get_cli_query() {
        let cli = Cli::from_args(vec![
            String::from("filename"),
            String::from("file_path"),
            String::from("query")
        ]);
        assert_eq!(*cli.query(), String::from("query"));
    }

    #[test]
    fn test_parse_no_query() {
        let args = vec![String::from("filename"), String::from("path")];
        let result = Cli::from_args(args);
        assert_eq!(*result.query(), "".to_string());
        assert_eq!(*result.file_path(), "path".to_string());
    }

    #[test]
    fn test_parse_full() {
        let args = vec![String::from("filename"), String::from("path"), String::from("query")];
        let result = Cli::from_args(args);
        assert_eq!(*result.query(), "query".to_string());
        assert_eq!(*result.file_path(), "path".to_string());
    }

    #[test]
    fn test_run() {
        let in_test = InputTest::default();
        let cli = Cli::from_args(vec![
            String::new(),
            in_test.file_path.clone(),
            String::from("has")
        ]);
        let result = run(&cli).unwrap();
        assert_eq!(result, String::from("the filthy fox has escaped"))
    }

    #[test]
    fn test_run_no_query() {
        let in_test = InputTest::default();
        let cli = Cli::from_args(vec![
            String::new(),
            in_test.file_path.clone(),
            String::new()
        ]);
        let result = run(&cli).unwrap();
        let expected = std::fs::read_to_string(&in_test.file_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_multiline() {
        let in_test = InputTest::default();
        let cli = Cli::from_args(vec![
            String::new(),
            in_test.file_path.clone(),
            String::from("fox")
        ]);
        let result = run(&cli).unwrap();
        let expected = std::fs::read_to_string(&in_test.file_path).unwrap();
        assert_eq!(result, expected);
    }

    struct InputTest {
        file_path: String
    }

    impl Drop for InputTest {
        fn drop(&mut self) {
            println!("Removing file {}", &self.file_path);
            std::fs::remove_file(&self.file_path).unwrap_or_default();
        }
    }
    impl Default for InputTest {
        fn default() -> Self {
            // create test file
            println!("Creating file test.txt");
            let file_path = "test.txt";
            let write_str = "The red fox going up\nthe black fox going down\nthe filthy fox has escaped";
            std::fs::write(file_path, write_str).unwrap();
            Self { file_path: String::from(file_path) }
        }
    }
}