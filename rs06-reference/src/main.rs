fn main() {
    // --- Reference Rules ---
    // 1. At any given time, you can have either one mutable reference or
    // any number of immutable references
    // 2. References must always be valid.

    let mut s = String::from("Hello World");
    let hello = &s[0..=4];
    let world = &s[6..=10];

    println!("{}-{}", hello, world);

    let world = first_word(&s);
    println!("{}", world);
    s.clear();

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    println!("{:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}