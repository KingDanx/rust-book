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

    let r3 = String::from("New string on the block");

    let new = &r3[..3];
    let string = &r3[4..=9];

    let r4 = String::from("testing second Thrid fourth fifth");

    println!("{new} {string}");

    println!("{}", first_word(&String::from("testing 1 2 3")));
    println!("{}", word_selector(&r4, 0));
    println!("{}", word_selector(&r4, 1));
    println!("{}", word_selector(&r4, 2));
    println!("{}", word_selector(&r4, 3));

    println!("{:?}", string_len(s4));

    let r3 = string_len(String::from("testing"));
    println!("{:?}", {r3});

    let a = [1, 2, 3, 4, 5];

    let b = &a[2..4];

    assert_eq!(b, &a[2..4]);

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

fn first_word(s: &str) -> &str { //&str is a string slice type
    let bytes = s.as_bytes();  //Converts a string into an array of bytes

    for (i, &item) in bytes.iter().enumerate() {  //iter returns each element in the collection.  enumerate() returns each element as part of a tuple instead.   //The first element of the tuple rturned from enumerate is the index, the second is a reference to the element
        if item == b' ' {  //b' ' a byte literal               
            return &s[..i];
        }
    }

    &s[..]
}

fn word_selector(s: &str, count: usize) -> &str {
    let bytes = s.as_bytes();

    let mut spaces = 0;
    let mut prev_space = 0;

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            spaces += 1;

            if spaces == count {
                prev_space = i;
            }

            if spaces > count {
                return &s[if count == 0 {prev_space} else {prev_space + 1}..i];
            }
        } 
    }

    &s[..]
}