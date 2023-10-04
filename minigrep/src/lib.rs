use std::fs;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Config {
        Config { 
            query,
            file_path,
        }
    }

    pub fn extract_file_text(&self) -> String {        
        fs::read_to_string(&self.file_path).unwrap_or_else(|err| {  //returns the value or the error
            println!("{}", format!("Problem extracting text from file {}: {err}", &self.file_path));
            process::exit(1); //format the error into a string and return on error
        })
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let config = Config::new(args[1].clone(), args[2].clone());

        Ok(config)
    }
}

pub fn run(args: &[String]) -> Config {
    let config = Config::build(args);
        
    let contents = match config {
        Ok(d) => {
            println!("Searching for {} in {}", d.query, d.file_path);
            // println!("{}", d.extract_file_text());
            d
        },
        Err(e) => {
            println!("Problem parsing the arguments: {}", e);
            process::exit(1);
        },
    };
    contents
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    if results.len() == 0 {
        println!("No results found");
    } else {
        println!("{:#?}", results);
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