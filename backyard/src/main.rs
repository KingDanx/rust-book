use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Backyard");

    let plant = Asparagus {};

    println!("I'm growing {:#?}", plant);
}
