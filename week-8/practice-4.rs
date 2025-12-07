use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move|| {
        tx.send("work complete").unwrap();
    });

    println!("message received: {}", rx.recv().unwrap());
}