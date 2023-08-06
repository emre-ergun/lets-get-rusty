#[allow(dead_code)]
#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese,
    Turkish,
}

fn main() {
    let language = Language::Japanese;

    // MATCH PATTERN
    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Turkish => println!("Merhaba Dünya!"),
        lang => println!("Unsupported language: {:?}", lang),
        //_ => println!("Unsupported language!"), // catch all pattern
    }


    // CONDITIONAL IF LET EXPRESSIONS
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _>= "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }


    // WHILE LET CONDITION
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // FOR LOOPS 
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // LET STATEMENTS
    // let PATTERN = EXPRESSION;
    let _x = 5;
    let (_x, _y, _z) = (1, 2, 3);

    // FUNCTION PARAMETERS
    let point = (3, 5);

    print_coordinates(&point);

    // Irrefutable
    // it can only accept irrefutable patterns:
    // function parameters
    // let statements
    // for loops
    let _x = 5;

    // refutable
    let x: Option<&str> = None;

    if let Some(x) = x {
        println!("{}", x);
    };

}

fn print_coordinates((x, y): &(i32, i32)) {
    println!("Current Location: ({}, {})", x, y);
}