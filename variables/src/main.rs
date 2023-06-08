fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //Shadowing example - a immutable variable can be changed to a different value or type without the mut keyword
    let spaces = "    ";  
    let spaces = spaces.len();  //Equivalent to .length() string method in JavaScript

    println!("The number of spaces is {spaces}");
}
