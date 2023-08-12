use std::error::Error;
use std::fs; //std::fs to handle files.

pub fn run(config: Config) ->Result<(),Box<dyn Error>> {
    // means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value.flexibility to return error values of different types in different error cases. The dyn keyword is short for “dynamic.”
    let contents = fs::read_to_string(config.file_path)?;//? will return the error value from the current function for the caller to handle.
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}//Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.
// creating a contructor for Config
// impl Config {
//     fn new(args: &[String]) -> Config {
//         //checking if the slicec is long enough before accessing elements
//         if args.len() <3 {
//             panic!("not enoough arggs")
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }
// }
//changing the above implementation into  RESULT<T,E> format
impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
        //checking if the slicec is long enough before accessing elements
        if args.len() <3 {
           return Err("not enough argss");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
