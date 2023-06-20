fn main() {

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    let s3 = gives_ownership();
    let mut s4 = takes_and_gives_back(gives_ownership());

    s2.push_str(" world");

    println!("{s1}, {s2}, {s3}, {s4}");

    ownership();

    println!("{} - {}", s4, reference_string_len(&s4));
    change_ref_string(&mut s4);
    {
        let r1 = &mut s4;
        r1.push_str(" cat?");
    }
    let r2 = &mut s4;


    r2.push_str(" car#");
    println!("{:?}", string_len(s4));
    


}

fn ownership() {
    let s = String::from("test");

    take_string(s);

    // println!("{s}"); this throws an error because take_string now owns s

    let x = 235;

    take_int(x);

    println!("ownership: {x}");  //this doesn't throw an error because numbers can be easily copied on the stack

}

fn take_string(string: String) {
    println!("{string}");
}

fn take_int(int: i32) {
    println!("take_int: {int}");
}

fn gives_ownership() -> String {
    let a_string = String::from("I am giving");

    a_string
}

fn takes_and_gives_back(mut string: String) -> String {
    string.push_str(" - I take and change");

    string
}

fn string_len(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}

fn reference_string_len(string: &String) -> usize {
    string.len()
}

fn change_ref_string(string: &mut String) {
    string.push_str(" dog!");
    println!("{}", string);
}