fn main() {
    let mut count = 0;
    
    let result = loop {
        count += 1;
        println!("{}", 10 - count);

        if count == 10 {
            break count * 2; //the break keyword here will return the expression after it.  Here that value would be stored in the result variable
        }
    }; //need semicolon after loop statment when assigning to a variable;
    println!("The result of the loop is: {result}");

    println!("The outter loop ran {} times", return_loop());

    while_loop();

    discount_for_loop([10, 20, 30, 40, 50]);

    for_loop();
}

//If you have loops within loops, break and continue apply to the innermost loop at that point.
fn return_loop() -> i32 {
    let mut count = 0;

    let result = 'counting_up: loop {
        println!("{count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count;
            }
            remaining -= 1;
        };
        count += 1;
    };
    println!("End count = {count}");
    result
}

fn while_loop() {
    let mut count = 3;

    while count >= 0 {
        println!("{count}");
        count -= 1;
    }

    println!("BLAST OFF!");
}

fn discount_for_loop(array: [i32; 5]) {
    let array_len = array.len();
    let mut count = 0;

    while count < array_len {
        println!("{}", array[count]);
        count += 1;
    }
}

fn for_loop() {
    let array = [8, 16, 32, 64, 128];

    for element in array {
        println!("The value is: {element}");
    }

    for number in (1..=3).rev() {
        println!("{number}");
    }
}

