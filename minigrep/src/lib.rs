use std::env::Args;
use std::fs;
use std::process;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Config {
        let ignore_case: bool = match env::var("IGNORE_CASE") {
            Ok(val) => string_to_bool(&val),
            Err(_) => false
        };

        println!("{ignore_case}");

        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn extract_file_text(&self) -> String {        
        fs::read_to_string(&self.file_path).unwrap_or_else(|err| {  //returns the value or the error
            eprintln!("{}", format!("Problem extracting text from file {}: {err}", &self.file_path));
            process::exit(1); //format the error into a string and return on error
        })
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let config = Config::new(query, file_path);

        Ok(config)
    }
}

pub fn run(args: Args) -> Config {
    let config = Config::build(args);
        
    let contents = match config {
        Ok(d) => {
            println!("Searching for {} in {}", d.query, d.file_path);
            // println!("{}", d.extract_file_text());
            d
        },
        Err(e) => {
            eprintln!("Problem parsing the arguments: {}", e);
            process::exit(1);
        },
    };
    contents
}

pub fn search(config: Config) {
    let file_contents = config.extract_file_text();
    if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents);
    } else {
        search_case_sensitive(&config.query, &file_contents);
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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

fn string_to_bool(string: &str) -> bool {
    let result: bool = match string.to_lowercase().trim() {
        "true" | "1" | "yes" => true,
        _ => false,
    };
    result
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

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}