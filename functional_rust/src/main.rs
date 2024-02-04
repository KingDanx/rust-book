use std::thread;
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Purple
}

#[derive(Debug)]
struct Shirt {
    stock: i32,
    color: ShirtColor,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<Shirt>,
}

impl Inventory {
    fn give_away(&mut self, customer_preference: Option<ShirtColor>) -> ShirtColor {
        customer_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&mut self) -> ShirtColor {
        let mut rng = rand::thread_rng();

        //? .iter() allows me to iterate over the vector.
        //? max_by_key allows me to find the max value by a givin value that all elements share
        //? it should return a single value as an option Some() or none.  The highest_stock is set to the Some() value if found or 0 if error
        let highest_stock = self.shirts.iter().max_by_key(|s| s.stock).map(|s| s.stock).unwrap_or(0);

        //? How to do a filter in rust. Since self.shirts is now reverse sorted I can take the first element and return 
        //? a Vec<_> of shirts that have the same stock as the most stocked
        let multiples: Vec<_> = self.shirts.iter().filter(|shirt| shirt.stock == highest_stock).collect();
        // println!("Multiples {:#?}", multiples);

        //? Return a random shirt from the multiples Vec<&Shirt>
        multiples[rng.gen_range(0..self.shirts.len() - 1)].color
    }
}


fn main() {
    println!("functional rust");
    let red_shirts = Shirt {
        stock: 200,
        color: ShirtColor::Red,
    };

    let blue_shirts = Shirt {
        stock: 100,
        color: ShirtColor::Blue,
    };

    let purple_shirts = Shirt {
        stock: 200,
        color: ShirtColor::Purple,
    };

    let mut store = Inventory {
        shirts: vec![red_shirts, blue_shirts, purple_shirts],
    };

    println!("{:#?}", store);


    let winner1 = Some(ShirtColor::Blue);
    let give_away1 = store.give_away(winner1);

    println!("{:#?}", give_away1);

    let winner2 = None;
    let give_away2 = store.give_away(winner2);

    println!("{:#?}", give_away2);

    // test_closure_borrow();

    // test_closure_borrow_thread();
}

fn test_closure_borrow() {
    let mut dog = String::from("Dog");
    
    println!("{dog}");
    
    let mut print_dog = || {
        println!("{dog}");
        dog = format!("{dog} cat")
    };

    print_dog();
    print_dog();
    print_dog();
    println!("{dog}");
}

fn test_closure_borrow_thread() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let one_thousand = |thread: &str| {
        let mut count = 0;
        loop {
            if count <= 1000 {
                println!("{thread}: {count}");
                count += 1;
            } else {
                break;
            }
        }
    };

    let spawned = thread::spawn(move || one_thousand("Spawn"));
    let spawned2 = thread::spawn(move || one_thousand("Spawn 2"));
    
    one_thousand("Main");

    spawned.join().unwrap();
    spawned2.join().unwrap();
}
