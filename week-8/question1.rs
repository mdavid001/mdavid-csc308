use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Start 3 threads
    for thread_id in 1..=3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let prefix = format!("T{}:", thread_id);

            for i in 1..=5 {
                let msg = format!("{} message {}", prefix, i);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(100)); // simulate work
            }
        });
    }

    // Drop the original sender so recv() will end after all clones close
    drop(tx);

    // Main thread prints messages as they arrive
    for received in rx {
        println!("Main received: {}", received);
    }

    println!("All messages received.");
}
