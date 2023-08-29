pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(number: i32) -> i32 {
    number + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be greater than 1 and less than or equal to 100 .  Got: {value}");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let result = add(2, 3);
        assert_eq!(result, 5);
        let result = add(2, 5);
        assert_eq!(result, 7);
    }

    #[test]
    fn exploration() {
        assert_eq!(3 + 3, 6);
    }
    
    #[test]
    fn failure() {
        if false {
            panic!("This shall fail!");
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Dog");
        assert!(
            result.contains("DoXg"),
            "Greeting did not include the NAME, VALUE WAS: {}, EXPECTED: {}", 
            result,
            greeting("DoXg")
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(555);
    }

    #[test]
    fn greater_than_one() {
        Guess::new(2);
    }

    #[test]
    fn it_work() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
