pub mod iomsg;

#[derive(Debug)]
struct Cli {
    query: String,
    file_path: String
}

fn parse_args(mut args: Vec<String>) -> Result<Cli, String> {
    // consumes the arguments to avoid unecessary copies
    let  arg_len = args.len();
    let query = if arg_len > 2  {
        args.remove(2) } else {
            iomsg::warn("Missing argument query, entire file will be printed");
            String::new()};
    let file_path = if arg_len > 1 {
        args.remove(1) } else {return Err(String::from("Missing file path"));};
    Ok(
        Cli {
            query: query.clone(),
            file_path:file_path.clone()
        }
    )
}

fn run(cli: &Cli) -> Result<String, std::io::Error>{
    let data = std::fs::read_to_string(&cli.file_path)?;
    if cli.query.is_empty() {
        return Ok(data);
    }
    for s in data[..].split('\n') {
        if s.contains(&cli.query) {
            return Ok(String::from(s));
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other,
        "Failed to find pattern"))
}

fn main() {
    // retrieve cli arguments
    let args = std::env::args().collect();
    // parse input arguments here
    let cli = parse_args(args).unwrap();     // args moved to cli
    // call run function
    match run(&cli) {
        Ok(x) => {iomsg::out(&x)},
        Err(e) => {iomsg::warn(&e.to_string());}
    };
}

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