struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn half_area(&self) -> u32 {
        self.width * self.height / 2
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("dogcatbird123"),
        email: String::from("dog@cat.bird"),
        sign_in_count: 1,
    };

    user1.email = String::from("cat@bird.dog");

    println!("{}", user1.email);

    let user2 = build_user(String::from("email@email.com"), String::from("username1"));

    let user3: User = User {
        active: user2.active,
        username: user1.username,
        email: String::from("bird@dog.cat"),
        sign_in_count: user2.sign_in_count,
    };

    let user4: User = User {
        email: String::from("dog@bird.cat"),
        ..user2
    };

    let white = Color(0, 0, 0);
    let black = Color(255, 255, 255);

    let zero = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // println!("{}", subject); //errors
    println!("{}", zero.1);
    println!("{}", black.1);
    println!("{}", white.0);
    println!("{}", user2.email);
    println!("{}", user3.email);
    println!("{}", user4.email);

    let width1 = 30;
    let length1 = 50;

    let dimensions = (20, 50);

    let rectangle2 = Rectangle {
        height: 40,
        width: 35,
    };

    let scale = 2;

    let rectangle3 = Rectangle {
        height: dbg!(30 * scale),
        width: 50,
    };

    let rectangle4 = Rectangle::square(3);

    dbg!(&rectangle3);

    println!("The area of rectangle 1 is {} square pixles.", area(width1, length1));
    println!("The area of rectangle 1 is {} square pixles.", area_tuple(dimensions));
    println!("The area of rectangle 1 is {} square pixles.", area_struct(&rectangle2));
    println!("The area of rectangle 1 is {} square pixles. Method", rectangle2.area());
    println!("The area of rectangle 1 is {} square pixles. Method half", rectangle2.half_area());
    println!("Rectangle 3 can fit inside of Rectangle 2: {}", rectangle3.can_hold(&rectangle2));
    println!("Rectangle 2 can fit inside of Rectangle 3: {}", rectangle2.can_hold(&rectangle3));
    println!("Rectangle 4 can fit inside of Rectangle 2: {}", rectangle2.can_hold(&rectangle4));
    println!("{:#?}", rectangle2);
    dbg!("{:#?}", &rectangle2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
