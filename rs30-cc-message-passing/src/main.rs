use std::sync::mpsc; //multi producer single consumer
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {

    });

    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    thread::spawn(move || {
        //let msg = String::from("hi");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        //let msg = String::from("hi");
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got: {}", received);
    }
    // let received = rx.recv().unwrap();
    // //let received = rx.try_recv().unwrap(); //does not block main thread. return result immediatelly
    // println!("Got: {}", received);
}