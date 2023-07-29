use std::fs::{self, File};
use std::io::{ErrorKind, self, Read};

fn main() {
    // NON-RECOVERABLE ERRORS
    // if you have unrecoverable error you can call panic macro
    //panic!("crash and burn");
    //panic with back_trace argument
    a();

    //RECOVERABLE ERRORS
    // Result enum is mostly used in Rust
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }
    let f  = File::open("hello.txt");

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("File could not opened!: {:#?}", error),
    // };

    //next step for file error
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Error while creating: {:#?}", err)
            },
            other_error => {
                panic!("Error while opening: {:#?}", other_error);
            },
        }
    };

    // more convenient way to open or create a file
    let _f = File::open("bye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("bye.txt").unwrap_or_else(|error| {
                panic!("Error while creating: {:#?}", error);
            })
        } else {
            panic!("Error while opening: {:#?}", error);
        }
    });

    let user_name = read_username_from_file().unwrap();
    println!("Username: {}", user_name.trim());
     
     let uname = read_uname_from_file().unwrap();
     println!("Uname: {}", uname.trim())
     
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; 
    Ok(s)
}

fn read_uname_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// backtrace example
fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("crash and burn");
    }
}
