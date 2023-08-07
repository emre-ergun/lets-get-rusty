extern "C" {
    fn abs(input: i32) -> i32;
}


static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    //unsafe rust
    // 1 - dereference raw pointer
    // 2 - call an unsafe fuction or method
    // 3 - access or modift a mutable static variable
    // 4 - implement an unsafe trait
    // 5 - Access fields of unions

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {} // body of unsafe function is unsafe block

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {

}

unsafe impl Foo for i32 {
    
}
