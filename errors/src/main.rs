use std::fs::{self, File};
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("src/hello.txt");  //?returns the file contents in the match chain

    match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    let greeting_file_result = File::open("src/hello.txt");  //?returns the file contents in the match chain

    match greeting_file_result {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => println!("File content: \n\t{}", content),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        },
        Err(_) => println!("Error opening file"),
    }

    // let greeting_file = File::open("hello.txt").unwrap();  //panics on error with system error message

    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project"); // panics on error with provided error message

    // println!("{:?}", greeting_file);

    println!("dog");
    
    // println!("{:?}", greeting_file_result);
    match read_username_from_file() {
        Ok(username) => println!("{}", username),
        Err(e)=> println!("{:?}", e), //?handle the error here
    }
    
    let username = read_username_from_file().unwrap(); //?value or panic
    
    println!("{username} unwrapped");

    let username = read_username_from_file_shortcut(); //?this function returns the error so username will be either a string or error but the error is not handled

    println!("{:?}", username);
    // println!("dog");

    match read_username_from_file_shortest() {
        Ok(username)=> println!("{username}"),
        Err(e)=> println!("{:?}", e),
    }

    // let greeting_file = File::open("hello.txt")?; //? this panics because File::open does not return a result or option

    let last_letter = last_char_of_first_line(""); //? can return the char or None

    println!("{:?}", last_letter);

    let last_letter = match last_char_of_first_line("") { //? None condiotion handled
        Some(last)=> last,
        None=> {
            let char: char = '0';
            char
        }
    };

    println!("{:?}", last_letter);

    let last_letter = match last_char_of_first_line("dog") {
        Some(last)=> last,
        None=> {
            let char: char = '0';
            char
        }
    };

    println!("{:?}", last_letter);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("src/hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), //?returns the error if there is one to where the function was called
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?; //? the ? will immediatly return the error value if the operation errors.

    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("src/hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}