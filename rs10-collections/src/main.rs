use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {

    // --- VECTORS ---
    let mut v1:Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![1, 2, 3, 4, 5];

    println!("v2[0]: {}", v2[0]);

    let third = &v2[2]; // problem is that it does not check boundries of array, may cause error.
    
    //v2.push(6); // error because one of the element of vector is borrowed immutable.
    
    println!("Third element is {}", third);

    match  v2.get(20) {
        Some(third) => {
            println!("The third element is {third}");
        },
        None => {
            println!("out of bounds");
        }
        
    }

    let mut v3 = vec![1, 2, 3, 4, 5];

    for elem in &mut v3 {
        *elem += 50;
    }

    for elem in &v3 {
        println!("{elem}");
    }

    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheet::Int(5),
        SpreadSheet::Float(5.5),
        SpreadSheet::Text("Hello".to_owned())
    ];

    match &row[1] {
        SpreadSheet::Float(f_number) => {
            println!("{f_number}");
        },
        _ => {
            println!("Not a float number");
        }
    }

    // --- STRINGS ---
    // strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("Initial contents");

    let mut s = String::from("foo");
    s.push_str(" bar");
    s.push('!');
    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = String::from(" World!");
    //let s3 = s1 + &s2; // s1 is moved
    let s3 = format!("{s1}{s2}"); 
    println!("{s3}");
    println!("{s1}{s2}"); // s1 and s2 are not moved , format! does not take ownership

    let hello = String::from("Hello");
    let chars = hello.chars();
    for ch in chars.into_iter() {
        println!("{ch}");
    }
    
    for g in hello.graphemes(true) {
        println!("{g}");
    }

    // --- HASH MAPS ---
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 5);
    scores.insert(yellow, 3);
    let team_name = String::from("blue");
    let _score = scores.get(&team_name).unwrap();

    for (key, value) in &scores {
        println!("{key}:{value}");
    }

    scores.insert(String::from("blue"), 50);
    scores.insert(String::from("blue"), 30); // overwrite it

    scores.entry(String::from("yellow")).or_insert(30);
    scores.entry(String::from("yellow")).or_insert(10);

    scores.entry(String::from("red")).or_insert(100);
    scores.entry(String::from("red")).or_insert(150);


    for (key, value) in &scores {
        println!("{key}:{value}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word)
                                .or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);

}
