use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Child 1: sleep 5
    let child_sleep = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn sleep");

    println!("sleep child PID: {}", child_sleep.id());

    // Child 2: ls -la
    let child_ls = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to spawn ls");

    println!("ls child PID: {}", child_ls.id());

    // Child 3: echo "Hello from child"
    let child_echo = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("Failed to spawn echo");

    println!("echo child PID: {}", child_echo.id());

    // Keep parent alive so we can inspect children
    println!("Parent sleeping for 10 seconds...");
    thread::sleep(Duration::from_secs(10));

}
