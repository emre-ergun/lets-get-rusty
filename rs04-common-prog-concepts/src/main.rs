fn main() {
    // ** common subjects ** 
    let mut x = 5;
    println!("x: {}", x);
    x = 6;  
    println!("x's new value: {}", x);

    let y = 10;
    println!("y: {}", y);
    let y = 15;
    println!("Shadowed y: {}", y);
    let y = "six";
    println!("Type changed 'y' by shadowing: {}", y);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("Subscribers: {}", SUBSCRIBER_COUNT);

    // ** Scalar Data Types ** 
    // Integers
    // Floating point numbers
    // Booleans
    // Characters
    let a = 98_222;
    let b = 0xff;
    let c = 0o777;
    let d = 0b1010_0011;
    let e = b'E';

    println!("{}-0x{:X}-0o{:o}-0b{:b}-{}", a, b, c, d, e as char);
    
    // ** Compound Data Types **
    // Tuples
    // Arrays

    let tup = ("Let's get rusty!", 100_000);
    println!("Tuple: {:?}", tup);
    let (channel, sub_count) = tup;
    println!("Subscriber: {}", sub_count);
    let sub_count = tup.1;
    println!("Channel: {}, Subscriber: {}", channel, sub_count);

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!("Not Found: {}", not_found); 

    let byte = [0; 8];
    println!("Bytes: {:?}", byte);

    // ** Functions **
    my_function();
    my_function_with_arguments(1, 2);
    let sum = my_function_with_arg_and_return(3, 5);
    println!("Result: {}", sum);

    // ** Control Flow **
    // ** if **
    // Conditions must be boolean in rust
    let number = 10;
    if number < 10 { 
        println!("first condition was true");
    } else if number > 10 {
        println!("second condition was true");
    } else {
        println!("none of conditions was true");
    }

    let condition = true;
    let number = if condition {5} else {10};
    println!("Number: {}", number);
    // ** loop **
    let mut counter = 0;
    let return_loop = loop {
        println!("{}.again!", counter + 1);
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("Loop result: {}", return_loop);

    // ** while **
    let mut number = 3;
     while number == 0 {
        println!("Number:{}", number);
        number -= 1;
     }
     println!("OUT of WHILE loop");

     // ** for **
     let a = [10, 20, 30, 40, 50];

     for element in a.iter() {
        println!("Elem: {}", element);
     }

     for number in 1..=5 {
        println!("Number: {}", number);
     }
}

fn my_function() {
    println!("Another function");
}

fn my_function_with_arguments(x: i32, y: i32) {
    println!("Arguments: {}-{}", x, y);
}

fn my_function_with_arg_and_return(x: i32, y: i32) -> i32 {
    x + y
    //or
    // return x + y;
}