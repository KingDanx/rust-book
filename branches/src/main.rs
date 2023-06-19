fn main() {
    let number = 7;

    if number != 0 {
        println!("number is not 0!");
    } else {
        println!("number is 0!");
    }

    more_conditions();

    println!("The returned value from my function is {}", if_assignment());
}

fn more_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisble by 4");
    } else if number % 3 == 0 {
        println!("Number is divisble by 3");        
    } else if number % 2 == 0 {
        println!("Number is divisble by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_assignment() -> i32 {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    number
}
//if statements in rust must always be supplied with a bool

//if statments are expressions so they return a value!

//Value types must be the same when they are assigning a value to a variable

//Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.

