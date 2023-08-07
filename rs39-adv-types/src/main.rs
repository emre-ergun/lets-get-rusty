fn main() {
    type Kilometers = i32;

    let x = 5;
    let y: Kilometers = 10;

    println!("x + y = {}", x + y);
}

//never type
fn bar() -> ! {
    // --snip--
}
