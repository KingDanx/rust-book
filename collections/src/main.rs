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
}
