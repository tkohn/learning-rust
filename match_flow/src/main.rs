use std::ops::Range;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    println!("Value: {}", value_in_cents(Coin::Penny));
    println!("Value: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    println!("{:?}", plus_one(Some(10)));
    println!("{:?}", plus_one(None));

    let dice_roll = 9;
    match dice_roll {
        3 => println!("You get a fancy hat!"),
        7 => println!("You loose a fancy hat!"),
        //other => println!("What ever {other} means."),
        // _ => println!("Nothing happens"), // '_' can be used it the value is not needed
        _ => (), // return unit value and ignore the dice_roll value
    }

    let opt: Option<String> = Some(String::from("Hello world"));
    match &opt {
        Some(s) => println!("{s:?}"),
        None => println!("None"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
