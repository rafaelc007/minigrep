use minigrep::{Cli, run};
use minigrep::iomsg;

fn main() {
    // retrieve cli arguments
    let args = std::env::args().collect();
    // parse input arguments here
    let cli = Cli::from_args(args);     // args moved to cli
    // call run function
    match run(&cli) {
        Ok(x) => {iomsg::out(&x)},
        Err(e) => {iomsg::warn(&e.to_string());}
    };
}