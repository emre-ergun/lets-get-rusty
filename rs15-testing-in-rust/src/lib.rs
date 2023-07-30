pub struct Rectangle {
    witdh: i32,
    height: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.witdh > other.witdh && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must greater than or equal to 1"
            );
        } else if value > 100 {
            panic!(
                "Guess value must smaller than or equal to 100 : {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            witdh: 8,
            height: 7,
        };
        let smaller = Rectangle {
            witdh: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            witdh: 8,
            height: 7,
        };
        let smaller = Rectangle {
            witdh: 5,
            height: 1,
        };

        assert!(smaller.can_hold(&larger) == false);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result =greeting("Emre");
        assert!(
            result.contains("Emre"),
            "greeting did not contain name, value was {}",
            result
            );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected="Guess value must greater than or equal to 1")]
    fn lesser_than_1() {
        Guess::new(-5);
    }
}
