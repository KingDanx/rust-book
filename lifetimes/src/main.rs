use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }
    
    // println!("r: {}", r);
    
    let string1 = String::from("long string is long!");
    let result;

    {
        let string2 = String::from("abc");
        result = longest(&string1[1..], string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i.part);    
    println!("{:?}", i.level());    

    let s: &'static str = "I have a static lifetime."; //? static lifetime will live the duration of the application.

    println!("{s}");

    println!("{:?}", longest_with_an_announcement(&string1, &first_sentence, "hi"))
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}