struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
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
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
