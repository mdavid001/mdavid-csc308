use std::process::Command;

fn main() {
    
    let child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to execute command");

    
    println!("Child PID:\n{}", child.id());
    
}