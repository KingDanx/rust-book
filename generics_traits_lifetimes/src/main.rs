#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

#[derive(Debug)]
struct MultiPoint<T, U> {
    x: T,
    y: U,
}

impl <T, U> MultiPoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }    
    fn mixup<T2, U2>(self, other: MultiPoint<T2, U2>) -> MultiPoint<T, U2> {
        MultiPoint { 
            x: self.x, 
            y: other.y 
        }
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest_i32(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest_i32(&number_list));

    let chars = vec!['a','b','c','0','d','e'];

    println!("{}", largest_char(&chars));
    
    let generic_i32 = largest(&number_list);
    
    println!("{}", generic_i32);
    println!("{}", largest(&chars));

    let integer_point = Point {
        x: 5,
        y: 20
    };

    let char_point = Point {
        x: 'a',
        y: 'b'
    };

    println!("{:?}", integer_point);
    println!("{:?}", char_point);
    
    let multi_point = MultiPoint {
        x: 10,
        y: 'a'
    };
    
    println!("{:?}", multi_point);
    println!("{:?}", multi_point.x());
    println!("{:?}", integer_point.x());
    
    let  multi_point2 = MultiPoint {
        x: "dog",
        y: String::from("cat"),
    };
    
    println!("{:?}", multi_point2);
    
    let multi_point3 = multi_point2.mixup(multi_point);
    println!("{:?}", multi_point3);
    

}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
