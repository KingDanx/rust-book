use std::fmt::Display;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

#[derive(Debug)]
struct MultiPoint<T, U> {
    x: T,
    y: U,
}

impl <T, U> MultiPoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }    
    fn mixup<T2, U2>(self, other: MultiPoint<T2, U2>) -> MultiPoint<T, U2> {
        MultiPoint { 
            x: self.x, 
            y: other.y 
        }
    }
}

// pub trait Summary {
//     fn summarize(&self) -> String; //? If there is no default like this you will need to handle it in every impl
// }
pub trait Summary {
    fn summarize_author(&self) -> String { 
        String::from("No author")
    }

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl <T, U> Summary for MultiPoint<T, U> {} //?empty block for default trait

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest_i32(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest_i32(&number_list));

    let chars = vec!['a','b','c','0','d','e'];

    println!("{}", largest_char(&chars));
    
    let generic_i32 = largest(&number_list);
    
    println!("{}", generic_i32);
    println!("{}", largest(&chars));

    let integer_point = Point {
        x: 5,
        y: 20
    };

    let char_point = Point {
        x: 'a',
        y: 'b'
    };

    println!("{:?}", integer_point);
    println!("{:?}", char_point);
    
    let multi_point = MultiPoint {
        x: 10,
        y: 'a'
    };
    
    println!("{:?}", multi_point);
    println!("{:?}", multi_point.x());
    println!("{:?}", integer_point.x());
    
    let  multi_point2 = MultiPoint {
        x: "dog",
        y: String::from("cat"),
    };
    
    println!("{:?}", multi_point2);
    
    let multi_point3 = multi_point2.mixup(multi_point);
    println!("{:?}", multi_point3);
    
    let my_articile = NewsArticle {
        headline: String::from("DANGER"),
        location: String::from("Germany"),
        author: String::from("R.L. Stien"),
        content: String::from("Are you feeling SHLAPPY?!"),
    };

    println!("{}", my_articile.summarize());

    let my_tweet = Tweet {
        username: String::from("KingDanx"),
        content: String::from("I'm feeling SHLAPPY!"),
        reply: true,
        retweet: false,
    };

    println!("{}", my_tweet.summarize());
    println!("{}", my_tweet.summarize_author());
    println!("{}", multi_point3.summarize());
    
    notify(&my_articile);
    
    let new = returns_summarizable();
    
    println!("{}", new.summarize());

    let pair = Pair::new(1, 2);
    let pair2 = Pair::new(2.1, 2.2);

    pair.cmp_display();
    pair2.cmp_display();

}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}