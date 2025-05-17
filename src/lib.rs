pub mod iomsg;

#[derive(Debug)]
pub struct Cli {
    query: String,
    file_path: String
}

impl Cli {
    pub fn query(&self) -> &String {&self.query}
    pub fn file_path(&self) -> &String {&self.file_path}
    pub fn from_args(mut args: Vec<String>) -> Self {
        let arg_len = args.len();
        let query = if arg_len > 2  {
            args.remove(2) } else {
                iomsg::warn("Missing argument query, entire file will be printed");
                String::new()
        };
        let file_path = if arg_len > 1 {
            args.remove(1) } else {
                iomsg::err("Missing file path");
                String::new()
        };
        Cli {
            query: query.clone(),
            file_path:file_path.clone()
        }
    }
}

pub fn run(cli: &Cli) -> Result<String, std::io::Error>{
    let data = std::fs::read_to_string(cli.file_path())?;
    if cli.query().is_empty() {
        return Ok(data);
    }
    let mut ret_str = String::new();
    for s in data[..].lines() {
        if s.contains(cli.query()) {
            if !ret_str.is_empty() {
                ret_str.push('\n');
            }
            ret_str += s;
        }
    }
    if ret_str.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
            "Failed to find pattern"))
    } else {
        return Ok(ret_str);
    }
}