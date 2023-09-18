use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // let mut file = File::create("example.txt")?;
    // file.write_all(b"Hello, world!")?;
    // Ok(())
    // let mut file = File::open("example.txt")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // println!("{}", contents);
    // Ok(())
    // let mut file = OpenOptions::new().write(true).open("example.txt")?;
    // file.write_all(b"Updated content")?;
    // Ok(())
    fs::remove_file("example.txt")?;
    Ok(())

}

