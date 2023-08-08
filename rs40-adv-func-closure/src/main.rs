fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
where T: Fn(i32) -> i32
{
    f(arg) + f(arg)
}
//Fn, FnMut, FnOnce
// Fn: closure captures the variables in its environment immutably
// FnMut: closure captures the variables in its environment mutably
// FnOnce: closure takes ownership of the variables in its environment
fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = 
        list_of_numbers
            .iter()
            .map(|i| i.to_string())
            .collect();

    println!("{:?}", list_of_strings);
    
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = 
        (0u32..20).map(Status::Value).collect();

    println!("{:#?}", list_of_statuses);
}

fn return_closure() -> impl Fn(i32) -> i32{
    |x| x + 1
}

fn return_box_closure(a: i32) -> Box<dyn Fn(i32) -> i32 >{
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a -b) 
    }
}
