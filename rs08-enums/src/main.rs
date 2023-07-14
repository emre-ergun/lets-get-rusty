use std::default;

enum IpAddrKind {
    V4(String),
    V6(u8, u8, u8, u8, u8, u8),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn _some_function() {
        println!("Lets get Rusty");
    }
}
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Arizona,
    Arkansas,
    California,
    //...
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // let _localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    let _localhost = IpAddrKind::V4(String::from("127.0.0.1"));

    // option enum
    // enum Option<T> {
    //     Some(T),
    //     None
    // }
    let _sum_number = Some(5);
    let _sum_string = Some("hi");
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(24);

    let _sum = x + y.unwrap_or(0);

    value_in_change(Coin::Quarter(UsState::California));

    println!("Plus one: {}", plus_one(Some(5)).unwrap_or(0));

    // if let pattern
    let some_five = Some(5);
    if let Some(5) = some_five {
        println!("five");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // instead of None we can use _ to handle all otherwise
        Some(i) => Some(i + 1)
    }
}
fn value_in_change(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => {
            println!("State is {:?}", us_state);
            25
        }
    }
}

fn _route(_ip_kind: IpAddrKind) {}