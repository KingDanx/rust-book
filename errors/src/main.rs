use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("src/hello.txt");  //returns the file contents in the match chain

    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/dog.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    let greeting_file_result = File::open("src/hello.txt");  //returns the file contents in the match chain

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

    // println!("{:?}", greeting_file_result);
}
