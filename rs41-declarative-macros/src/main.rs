#[macro_export]
macro_rules! vec1 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec!["a", "b", "c", "d", "e"];

    let v3 = vec1!(1, 2, 4, 5);
    println!("{:?}", v3);
    
}
