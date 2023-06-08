fn main() {

    //Integers and mathmatics:

    /*
    | Length|Signed| Unsigned |
    | 8-bit	 |  i8   |	u8    |
    | 16-bit |  i16  |	u16   |
    | 32-bit |  i32  |	u32   |
    | 64-bit |  i64  |	u64   |
    | 128-bit|  i128 |	u128  |
    |  arch  | isize |	usize |  <- based on system archetecture
     */

    //Declaring a float without a type annotation yields a f64 by default 
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    //additition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    //remainder
    let remainder = 43 % 5;

    println!("Sum: 5 + 10 = {sum}");
    println!("Difference: 95.5 - 4.3 = {difference}");
    println!("product: 4 * 30 = {product}");
    println!("Quotient: 56.7 / 32.2 = {quotient}");
    println!("Truncated: -5 / 3 = {truncated}");
    println!("Remainder: 43 % 5 = {remainder}");


    //Booleans:

    let t = true;

    let f: bool = false; //with explicit type annotation

    println!("5 + 10 = {sum}? {t}");
    println!("5 + 10 = {difference}? {f}");


    //Char data type: 4 bytes

    let c = 'z';
    let z: char = 'e'; //with explicit type annotation
    let smile = '☻';

    println!("{z}{c} {smile}");

    //Compound types:

    //Tuple Type: Can have different types
    let tup: (i32, f64, bool) = (500, -2.7, true);

    let (t1, t2, t3) = tup;

    println!("{t1}, {t2}, {t3}, {}, {}, {}", tup.0, tup.1, tup.2);

    //Arrays: Must all have the same type
    let my_array = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let define_array: [i32; 5/*number of elements */] = [1,2,3,4,5];
    let fill_array = [0 /*array will fill with 0's */; 5/*number of elements */];

    println!("{} {}, {}{}{}{}", months[0],define_array[0],my_array[1],fill_array[4],define_array[1],my_array[2]);
}
