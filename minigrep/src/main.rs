use std::env;
use std::fs;

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
        fs::read_to_string(self.file_path).expect("why no poem?")
    }
}


fn main() {
    let args: Vec<String> = env::args().collect(); 
    println!("Hello, world!");
    dbg!(&args);

    if args.len() < 3 {
        panic!("Error: Please provide arguments for -- [search criteria] [file name]");
    }

    let config = parse_config(&args);

    
    println!("{:?}", config);
    println!("{}", config.extract_file_text());
}

fn parse_config(args: &[String]) -> Config {
    Config::new(args[1].clone(), args[2].clone())
}
