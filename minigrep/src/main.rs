use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(query: String, file_path: String) -> Config {
        Config { 
            query,
            file_path,
        }
    }

    fn extract_file_text(self) -> String {        
        fs::read_to_string(self.file_path).unwrap_or_else(|err| {  //returns the value or the error
            format!("Problem parsing arguments: {err}") //format the error into a string and return on error
        })
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let config = Config::new(args[1].clone(), args[2].clone());

        Ok(config)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect(); 
    // dbg!(&args);    
    run(&args);
}

fn run(args: &[String]) {
    let config = Config::build(args);
        
    let contents = match config {
        Ok(d) => {
            println!("Searching for {} in {}", d.query, d.file_path);
            d
        },
        Err(e) => {
            println!("Problem parsing the arguments: {}", e);
            process::exit(1);
        },
    };
    println!("{}", contents.extract_file_text());
}
