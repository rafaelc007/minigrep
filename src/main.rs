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

fn run() {

}

fn main() {
    // retieve cli arguments
    let args = std::env::args().collect();
    // parse input arguments here
    let cli = parse_args(args);     // args moved to cli
    // call run function
    run();
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
}