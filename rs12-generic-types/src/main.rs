#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { 
            x: self.x, 
            y: other.y,
        }
    }
}

#[allow(dead_code)]
struct Nokta<T> {
    x: T,
    y: T,
}

impl<T> Nokta<T> {
    fn x(&self) -> &T {
        &self.x
    }
 }

 impl Nokta<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}
// Option enum and Result enum are implemented by using generic types

fn main() {
    let number_list = vec![34, 55, 100, 25, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![100, 200, 150, 6000, 250, 50];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['a', 'z', 'r', 'k', 'v', 's'];
    let largest = get_largest(char_list);
    println!("The largest char is {}", largest);

    let p1  = Point {
        x: 1.1,
        y: 2.2
    };

    let p2 = Point {
        x: 5,
        y: 10,
    };

    let p3 = Point {
        x: 5,
        y: 3.3
    };

    let _p4 = p1.mixup(p2);
    let _p5 = p3.mixup(Point { x: 3.3, y: 6 });

    let p4 = Nokta {
        x: 5.0,
        y: 10.0
    };
    println!("Nokta x: {}", p4.x());
    println!("Nokta y: {}", p4.y());

    let _integer = Option::Some(5);
    let _float = Option::Some(4.4);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

