pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn deliver_order() {}

// use crate::front_of_house::hosting; //essetially pulls hosting to the crate root.
use crate::front_of_house::hosting as dog; //essetially pulls hosting to the crate root and names it differently.
pub use crate::front_of_house::hosting as pubHost; //I think allows for this to be used in something outside the module.  The doc was very vauge and showed no example.
use crate::front_of_house::hosting::{self, add_to_waitlist}; //allows you to import multiple modules from the same parent module and include that module
// use crate::front_of_house::{self, serving}; //allows you to import multiple modules from the same parent module.
use crate::back_of_house::*; //brings all public items into scope


mod front_of_house {
    pub mod hosting {
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {

    dog::add_to_waitlist();

    hosting::add_to_waitlist();

    pubHost::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    let mut meal2 = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

mod back_of_house {
    pub enum Appetizer {  //Enums have their fields public by default.
        Soup,
        Salad,
    }

    pub struct  Breakfast { //Structs have their fields private by default like most other things.
        pub toast: String,
        seasonal_fruits: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruits: String::from("peaches") ,
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  //super is equivalant to ../  this moves it one module up. So it would be in the lib.rs base module (crate) where it can find deliver_order()
    }

    fn cook_order() {}
}
