fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labled_measurements(5, '☻');
}

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labled_measurements(value: i32, unit_label: char){
    println!("The mesasurement is: {value}{unit_label}");
}