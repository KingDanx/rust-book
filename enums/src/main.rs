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
}
