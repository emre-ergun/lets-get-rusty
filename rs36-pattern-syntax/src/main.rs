fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // catch all
    }

    let x = Some(5);
    let _y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // y shadow y defined outside of the match above
        Some(y) => println!("Mathced, y = {:?}", y),
        _ => println!("default case, x = {:?}", x),
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII characters"),
        'k'..='z' => println!("late ASCII charactes"),
        _ => println!("something else"),
    }

    let p = Point {
        x: 0,
        y: 7,
    };

    let Point { x: a, y: b} = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point {x, y: 0} => {
            println!("on the x axis at {}", x)
        },
        Point { x: 0, y } => {
            println!("on the y axis at {}", y)
        },
        Point { x, y } => {
            println!("on the axis: ({}, {})", x, y)
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        },
        Message::Move { x, y } => {
            println!(
                    "Move to x: {} y: {}",
                    x, y
            );
        },
        Message::Write(text) => {
            println!("Text message: {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change color to red:{}, green:{}, blue:{}",
                r, g, b
            )
        }
    }

     let origin = Point {
        x: 0,
        y: 0,
     };

     match origin {
         Point { x, .. } => println!("x is {}", x),
     }

     // _ and _x are not same. there is still binding for _x

     let numbers = (2, 4, 6, 8, 10, 12);

    match numbers {
        (first, .. , last) => {
            println!("Some numbers: {}-{}", first, last);
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less then five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("default case, x = {:?}", x),
     }

     let msg = Msg::Hello { id: 5 };

     match msg {
        Msg::Hello { 
            id: id_variable @ 3..=7 
        } => println!("Found and id in range: {}", id_variable),
        Msg::Hello { 
            id: 10..=12
        } => println!("Found and id in another range"),
        Msg::Hello { id } => println!("Found some other id: {}", id),
     }
}

struct Point {
    x: i32,
    y: i32
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Msg {
    Hello{ id: i32},
}