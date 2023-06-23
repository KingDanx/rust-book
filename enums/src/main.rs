#[derive(Debug)]
struct Ipv4Addr {
    address: (u8, u8, u8, u8),
    protocol: String,
}
#[derive(Debug)]
struct Ipv6Addr {
    address: String,
    protocol: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddrKind {
    fn log_protocol(&self) {
        println!("{:#?}", self)
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called");
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("How LUCKY!?");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quater(state) => {
                println!("State quater from {:?}", state);
                25
            },
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(Ipv4Addr {
        address: (127, 0, 0, 1),
        protocol: String::from("Ipv4"),
    });

    let loopback = IpAddrKind::V6(Ipv6Addr {
        address: String::from("::1"),
        protocol: String::from("Ipv6"),
    });

    let m = Message::Write(String::from("Hello there"));
    m.call();

    home.log_protocol();
    loopback.log_protocol();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{:#?}", some_number);
    println!("{:#?}", some_char);
    println!("{:#?}", absent_number);

    let penny = Coin::Penny;
    println!("{}", penny.value_in_cents());

    let quater = Coin::Quater(UsState::Alaska);
    println!("{}", quater.value_in_cents());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:#?}", six);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => { //other here can be named anything and I guess any variable that isn't defined already.  Will cover any arm that isn't specified.
            println!("{:?}", other);
            move_player(other)
        }, 
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => { //Use this if you are just going to throw away the value and don't plan to use it at all in the arm.
            reroll()
        }, 
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //Do nothing, I guess the same as just saying return?
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //Alternate for the above code
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {
    println!("REROLLING!");
}