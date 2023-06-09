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

    // let first = &v[0]; //this is an immutable borrow
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
            SpreadsheetCell::Int(x) => println!("{x}"),
            SpreadsheetCell::Float(x) => println!("{x}"),
            SpreadsheetCell::Text(x)=> println!("{x}"),
            _ => println!("Not the droid you're looking for."),
        }
    }


}

fn take_num(x: i32) {
    println!("I take {}", x);
}