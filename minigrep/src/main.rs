use std::env;
use minigrep;


fn main() {
    let config = minigrep::run(env::args());
    
    minigrep::search(config);
 
}
