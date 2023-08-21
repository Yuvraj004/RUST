use std::error::Error;
use std::fs; //std::fs to handle files.
use std::env;

pub fn run(config: Config) ->Result<(),Box<dyn Error>> {
    // means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value.flexibility to return error values of different types in different error cases. The dyn keyword is short for “dynamic.”
    let contents = fs::read_to_string(config.file_path)?;//? will return the error value from the current function for the caller to handle.
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case : bool,//configuration option to the Config struct to 
    //switch between case-sensitive and case-insensitive search.
    //
    //
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
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        //the var function from the env module to check to see if any value has been set for an environment variable named IGNORE_CASE
        Ok(Config { query, file_path,ignore_case })
    }
}

//after using iterators
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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

//search  case insensitive function
pub fn search_case_insensitive<'a>(
    query : &str,
    contents: &'a str,
)  -> Vec<&'a str>{

    let query = query.to_lowercase();// calling to_lowercase creates new data rather than referencing existing data therefore query is now a string not a string slice
    let mut results = Vec::new();


    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}
