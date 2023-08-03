use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x); // value of x is copied

    assert_eq!(5, x);
    assert_eq!(5, *y); //dereference operator *
    assert_eq!(5, *z);

    let a = 5;
    let b = MyBox::new(a);

    assert_eq!(5, *b);
    assert_eq!(5, *(b.deref())); // same as previous line

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str (automatic deref coercion)
    // if there was no auto coercion, we would use it like
    // &(*m)[..] -> &str
    hello(&(*m)[..]);

}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
