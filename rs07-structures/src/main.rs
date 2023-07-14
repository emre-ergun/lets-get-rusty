#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("emre-ergun"),
        email: String::from("emreergun@engramsoft.com"),
        sign_in_count: 0,
        active: true,
    };

    let name = &mut user1.username;
    *name = String::from("ergun-emre");
    println!("{}", user1.username);

    let user2 = build_user(
        String::from("Hello"),
        String::from("pvt.emreergun@gmail.com"),
    );

    let _user3 = User {
        username: String::from("e.ergun"),
        email: String::from("emrergun61@gmail.com"),
        ..user2
    };

    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let color1 = Color(1, 2, 3);
    let _point1 = Point(0, 0, 0);

    let (_r, _g, _b) = (color1.0, color1.1, color1.2);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 15,
        height: 25,
    };

    println!("Rectangle: {:#?}", rect);
    println!("Area by separated function: {}", area(&rect));
    println!("Area by impl function: {}", rect.area());

    println!("{:?} can hold {:?}: {}", rect, rect1, rect.can_hold(&rect1));

    let new_square = Rectangle::square(10);
    println!("Square: {:#?}", new_square);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}
