use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter protected by Mutex, shared using Arc
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 3;
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print result
    println!("Final counter value = {}", *counter.lock().unwrap());
}
