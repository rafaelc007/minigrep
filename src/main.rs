use minigrep::{Cli, run};
use minigrep::iomsg;

fn main() {
    // parse input arguments here
    let cli = Cli::from_args(std::env::args());     // args moved to cli
    // call run function
    match run(&cli) {
        Ok(x) => {iomsg::out(&x)},
        Err(e) => {iomsg::err(&e.to_string());}
    };
}