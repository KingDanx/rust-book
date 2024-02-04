#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

#[derive(Debug)]
struct Shirt {
    stock: u32,
    color: ShirtColor,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<Shirt>,
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

    let store = Inventory {
        shirts: vec![blue_shirts, red_shirts],
    };

    println!("{:#?}", store);


    let winner1 = Some(ShirtColor::Blue);
    let give_away1 = store.give_away(winner1);

    println!("{:#?}", give_away1);

    let winner2 = None;
    let give_away2 = store.give_away(winner2);

    println!("{:#?}", give_away2);
}

impl Inventory {
    fn give_away(&self, customer_preference: Option<ShirtColor>) -> ShirtColor {
        customer_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut max = 0;

        for shirt in &self.shirts {
            if shirt.stock > max {
                max = shirt.stock
            }
        }    

        let most_stocked = self.shirts.iter().find(|el| el.stock == max);

        let shirt = most_stocked.unwrap_or(&Shirt {color: ShirtColor::Blue, stock: 0});

        shirt.color
    }
}
