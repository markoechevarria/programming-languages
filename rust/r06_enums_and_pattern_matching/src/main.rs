/*
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
*/

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

enum Coin {
    Penny, Nickel, Dime, Quarter
}

fn main() {

    let _home = IpAddrKind::V4(127,0,0,1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let message_instance = Message::Write(String::from("This is a message"));
    message_instance.call();
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    

    let some_number: Option<usize> = Some(5);
    let _some_char: Option<char> = Some('e');
    let _absent_number: Option<i32> = None;
    let other_number: usize = 8;

    // println!("{}", some_number + other_number );
    println!("{}", some_number.unwrap() + other_number );


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_places(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        _ => (),
    }

    let config_max = Some(3u8);
    let mut count = 0;

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => count += 1,
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        count += 1;
    }
}

// fn route(ip_kind: IpAddrKind) {
//    println!("{:#?}", ip_kind);
// }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        } ,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_places(n: usize) {}
fn reroll() {}
