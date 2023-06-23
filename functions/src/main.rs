fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labled_measurements(5, '☻');
    expresion_test(plus_one(six()), five())
}

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labled_measurements(value: i32, unit_label: char){
    println!("The mesasurement is: {value}{unit_label}");
}

fn expresion_test(value: i8, another_value: i8){
    let expression_value: i8 = {
        let third_value: i8 = 7;
        third_value + value + another_value // This line does not have a semi-colon because that would make it a statement and would then not return a value.
    };

    println!("Expression test: {expression_value}");
}

//We don’t name return values, but we must declare their type after an arrow (->)
fn five() -> i8 {
    5
}

//You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
fn six() -> i8 {
    return 6
}

fn plus_one(number: i8) -> i8 {
    number + 1
}