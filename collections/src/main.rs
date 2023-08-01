#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let v: Vec<i32> = Vec::new(); //defining an empty vector, needs a type assignment Vec<T>
    println!("{:#?}", v);
    
    let v = vec![1, 2, 3];  //a macro that creates a vector with inferred types
    println!("{:#?}", v);
    let v = vec!["one", "two", "three"];  //a macro that creates a vector with inferred types
    println!("{:#?}", v);
    
    let mut v = Vec::new();  //defines a new epmty vector that is mutable.  Type is not needed since it is empty and mutable?
    v.push(1);       //Pushed values must all be the same type  
    v.push(12);      //Pushed values must all be the same type
    v.push(123);     //Pushed values must all be the same type
    v.push(1234);    //Pushed values must all be the same type
    v.push(12345);   //Pushed values must all be the same type
    v.push(1234);    //Pushed values must all be the same type
    v.push(123);     //Pushed values must all be the same type
    v.push(12);      //Pushed values must all be the same type
    v.push(1);       //Pushed values must all be the same type  
    println!("{:#?}", v);
    
    let third: i32 = v[2]; //get a value from a vector.
    println!("{:#?}", third);
    take_num(v[2]);  //aparently this does not follow the borrowing rules as I can send it to a function and still access it.
    println!("{:#?}", v[2]); //The whole vector would have to be passed to trigger the borrowing rules it appears
    
    let third: Option<&i32> = v.get(2); // this .get() returns an Option 
    match third {
        Some(x) => println!("The third element of the vector is {}", x),
        None => println!("There is no third element!"),
    }

    let index = 54;
    let does_not_exist = v.get(index);
    match does_not_exist {
        Some(x) => println!("The value at this index is {x}"),
        None => println!("Program paniced because of an out of bounds error, the vector is only {} elements long don't ya know\r\nYou entered {index}", v.len()),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i *= 20;
        println!("{i}");
    }
    
    let first = v[0]; 

    // let first = &v[0]; //this is an immutable borrowed reference.
    v.push(6); //The code won't compile because the immutable reference's memeory location could have changed as a result of adding a value to the vector so it has become invalidated. 

    println!("The first element is {first}"); 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.5),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:?}", row);

    for i in &row {
        match i {
            SpreadsheetCell::Int(x) => println!("Found int in enum vector: {x}"),
            SpreadsheetCell::Float(x) => println!("Found float in enum vector: {x}"),
            SpreadsheetCell::Text(x)=> println!("Found text in enum vector: {x}"),
        }
    }

    let s = String::new();

    println!("{s}");

    let data = "initial contents";

    let s = data.to_string();

    println!("{s}");
    
    let s = "secondary contents".to_string();
    
    println!("{s}");
    
    let s = String::from("tertiary contents");
    
    println!("{s}");
    
    let mut s = String::from("foo");
    
    s.push_str("bar");
    
    println!("{s}");
    
    let mut s1 = String::from("bar");
    let s2 = "foo";
    
    s1.push_str(s2);

    println!("{s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    let hello = String::from("hello");

    let ello = &hello[1..hello.len()]; //string slice from selected index to the end of the string

    println!("{ello}"); 

    println!("{}", &ello[0..=0]);

    //iterating over a string
    let mut reverse_me = String::new();
    for (index, c) in hello.chars().enumerate() {
        reverse_me.push_str(&hello[hello.len() - (index + 1)..=hello.len() - (index+ 1)]);
        println!("{} - {}", reverse_me, index);
    }

    

}

fn take_num(x: i32) {
    println!("I take {}", x);
}