use std::env;
use minigrep;


fn main() {
    let args: Vec<String> = env::args().collect(); 
    // dbg!(&args);    
    minigrep::run(&args);
}
