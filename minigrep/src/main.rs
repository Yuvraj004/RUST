use std::env;//function provided in Rust’s standard library. This function returns an iterator of the command line arguments passed to minigrep
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::build(&args).unwrap_or_else(|err|{
        println!("problem parsing args: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //run(config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

