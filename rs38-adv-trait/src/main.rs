pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

// difference between Generic Types and Associated Types(Item)
// -- associated types can only have one concrete type
// per implementation

struct Counter {

}

impl Iterator for Counter {
    type Item = u32; // we can't have another implementation

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter1 {}

impl Iterator1<u32> for Counter1{
    fn next(&mut self) -> Option<u32> {
        Some(10)
    }
}

impl Iterator1<u16> for Counter1{
    fn next(&mut self) -> Option<u16> {
        Some(10)
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


struct Milimeters(u32);
struct Meters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, rhs: Meters) -> Milimeters {
        Milimeters(self.0 + (rhs.0 * 1000))
    }
}


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", "*".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", "*".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point1 {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point1 {
}

impl fmt::Display for Point1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {

    assert_eq!(
        Point {x: 1, y: 0} + Point {x: 2, y: 3},
        Point {x: 3, y: 3}
    );

    let person = Human;

    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);

    let w = Wrapper (
        vec![String::from("hello"), String::from("world")]
    );
    println!("v: {}", w);
}
