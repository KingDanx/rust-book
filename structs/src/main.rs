fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let mut user1 = User {
        active: true,
        username: String::from("dogcatbird123"),
        email: String::from("dog@cat.bird"),
        sign_in_count: 1,
    };

    user1.email = String::from("cat@bird.dog");

    println!("{}", user1.email);

}
