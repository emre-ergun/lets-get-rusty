use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("counter after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("counter after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        //Rc::clone() method does not do deep copy
        // it only increments the reference counter  
        println!("counter after creating c = {}", Rc::strong_count(&a));
    }
    println!("counter after c goes out of scope = {}", Rc::strong_count(&a));

}
