pub mod iomsg;

#[derive(Debug)]
pub struct Cli {
    query: String,
    file_path: String
}

pub fn parse_args(mut args: Vec<String>) -> Result<Cli, String> {
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

pub fn run(cli: &Cli) -> Result<String, std::io::Error>{
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