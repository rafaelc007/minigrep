use minigrep::{Cli, run, parse_args}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_query() {
        let args = vec![String::from("filename"), String::from("path")];
        let result = parse_args(args).unwrap();
        assert_eq!(result.query, "".to_string());
        assert_eq!(result.file_path, "path".to_string());
    }

    #[test]
    fn test_parse_full() {
        let args = vec![String::from("filename"), String::from("path"), String::from("query")];
        let result = parse_args(args).unwrap();
        assert_eq!(result.query, "query".to_string());
        assert_eq!(result.file_path, "path".to_string());
    }

    #[test]
    fn test_run() {
        let in_test = InputTest::default();
        let cli = Cli {
            query: String::from("has"),
            file_path: in_test.file_path.clone()};
        let result = run(&cli).unwrap();
        assert_eq!(result, String::from("the filthy fox has escaped"))
    }

    #[test]
    fn test_run_no_query() {
        let in_test = InputTest::default();
        let cli = Cli {
            query: String::new(),
            file_path: in_test.file_path.clone()};
        let result = run(&cli).unwrap();
        let expected = std::fs::read_to_string(&in_test.file_path).unwrap();
        assert_eq!(result, expected)
    }

    struct InputTest {
        file_path: String
    }

    impl Drop for InputTest {
        fn drop(&mut self) {
            std::fs::remove_file(&self.file_path).unwrap();
        }
    }
    impl Default for InputTest {
        fn default() -> Self {
            // create test file
            let file_path = "test.txt";
            let write_str = "The red fox going up\nthe black fox going down\nthe filthy fox has escaped";
            std::fs::write(file_path, write_str).unwrap();
            Self { file_path: String::from(file_path) }
        }
    }
}