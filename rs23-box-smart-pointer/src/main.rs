enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));

    // Box Smart Pointers
    // Reference is most common pointer in rust
    // Reference borrows the value it points to
    // They don't have ownership over the values
    // Smart pointers are data structures act like pointer
    // but have meta data and extra capabilities
    // Most cases smart pointers own the data that they point to
    // for example Strings and vectors are smart pointers
    // Smart pointers are structures that they implement deref 
    // and drop traits
    // deref traits: allows instances of smart pointer structs to be 
    // traited like references
    // drop traits: allows you to customize the code that is run when
    // the smart pointer goes out of scope
    let b = Box::new(5);
    println!("b = {}", b);
}



