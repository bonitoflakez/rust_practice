#![allow(unused)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    {
        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V4(String::from("::1"));
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
            // other => move_player(other),
        }
    }

    // concise control flow with if let
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("maximum is configured to be {}", max),
            _ => (),
        }
    }

    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("maximum is configured to be {}", max);
        }
    }
}

fn reroll() {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
