
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("unknown")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking News! {}", item.summarize());
// }
// more convinent implementation
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}


use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number x is {}", self.x);
        } else {
            println!("The largest number y is {}", self.y);
        }
    }
}
fn main() {
    // traits
    let tweet = Tweet {
        username: String::from("@engramsoft"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Emre Ergün"),
        headline: String::from("The Sky is Falling"),
        content: String::from("The sky is not actually falling"),
    };

    println!("{}", tweet.summarize());
    println!("{}", article.summarize());

    notify(&tweet);
    notify(&article);

}
