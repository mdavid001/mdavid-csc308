use std::thread;

fn main(){
    let message = String::from("Hello from the spawned thread!");

    let handler = thread::spawn(move || {
        println!("{}", message);
    });
    handler.join().unwrap();
}