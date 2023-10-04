use std::env;
use minigrep;


fn main() {
    let args: Vec<String> = env::args().collect(); 
    // dbg!(&args);    
    let config = minigrep::run(&args);
    let file_contents = config.extract_file_text();
    
    minigrep::search(&config.query, &file_contents);
 
}
