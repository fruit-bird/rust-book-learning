#![allow(dead_code, unused_variables, unused_assignments)]

#[derive(Debug)]
enum UsState {
    Florida,
    NewYork,
    California,
    NorthCarolina,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn run() {
    let coin = Coin::Quarter(UsState::Florida);
    println!("{}", coin.value_in_cents());

    let five = Some(5);
    let six = plus_one(Some(6));
    let none = plus_one(None);

    if let Some(5) = five {
        println!("if let got brred");
    }
    let mut counter = 0;
    if let Coin::Quarter(state) = &coin {
        println!("Quarter from {:?}", coin);
    } else {
        counter += 1;
    }
}

fn odd_match(some_u8_value: Option<u8>) {
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        Some(9) => println!("nine"),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky Penny!!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State Quarter from {:?}!", state);
                25
            }
        }
    }
}
