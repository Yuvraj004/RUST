use std::env;//function provided in Rust’s standard library. This function returns an iterator of the command line arguments passed to minigrep
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    // let config= Config::build(&args).unwrap_or_else(|err|{
    //     eprintln!("problem parsing args: {err}");//for printing errors directly
    //     process::exit(1);
    // });

        //now we are directly passing  ownership of the iterator returned to config::build RATHER THAN PAASSING A SLICE OF THE VECTOR STORED ITERATOR VALUES
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //run(config);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
//To redirect the output sttrream into a file
//$ cargo run > output.txt
