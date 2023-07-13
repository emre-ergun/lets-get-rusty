fn main() {
    // ----- Ownership Rules -------
    // 1. Each value in Rust has a variable that is called its owner.
    // 2. There can be only one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // a value has only one mutable reference at a time.
    // if a value has an immutable reference, it can not have mutable reference

    {// s is not valid here, it is not declared yet
        let s = String::from("Hello"); //s is valid from this point
        println!("{}", s);
        // do stuff with s   
    }// this scope is now over, and s is no longer valid.

    let x = 5;
    let _y = x; // Copy

    let s1 = String::from("move");
    let s2 = s1; //move (not a shallow copy)
    // println!("{}, world!", s1); // error because s1 is borrowed at pervious line
    println!("{}, world!", s2);
    let s3 = s2.clone(); //cloned, as new variable
    println!("{}, world!", s3);

    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s); //error because takes_ownership() takes the ownership

    let x = 6;
    makes_copy(x);
    println!("{}", x);

    let get_ownership = gives_ownership();
    println!("{}", get_ownership);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}:{}", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);


    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}

fn change(some_string: &mut String) {
    some_string.push_str(" World!");
}
fn calculate_length(some_string: &String) -> usize {
    let length = some_string.len();
    length
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(val: i32) {
    println!("{}", val);
}

fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}