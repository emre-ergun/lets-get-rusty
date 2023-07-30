use std::fmt::Display;

struct ImportantExperct<'a> {
    part: &'a str,
}

impl<'a> ImportantExperct<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Emre. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find!");
    let i = ImportantExperct {
        part: first_sentence,
    };

    println!("First sentence: {}", i.return_part("!!!!!"));

    let s: &'static str = "Hello there!";
    println!("{}", s);
    // let r;

    // {
    //     let x = 5;
    //     r = &x; // it give eerror because of borrow checker 
    // }

    // println!("r: {}", r); // r is dangling pointer

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Look at me!");
    println!("Result: {}", result);
    
    let mut result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    {
        let string2 = String::from("xyz");
        //result = longest(string1.as_str(), string2.as_str()); //error because strin2 does not live long enough
        result = longest1(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// generic lifetime annotation
// &i32         a reference
// &'a i32      a reference with an explicit lifetime
// &'a mut i32  a mutable reference with an explicit lifetime
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn longest1<'a>(str1: &'a str, _str2: &str) -> &'a str {
    str1
}


