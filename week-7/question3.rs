use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn long-running ping process
    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::null()) // optional: keep terminal clean
        .spawn()
        .expect("Failed to spawn ping");

    println!("Spawned ping with PID: {}", child.id());
    println!("Now open another terminal and run: top  OR  ps aux | grep ping");

    // Wait 5 seconds while ping is running
    thread::sleep(Duration::from_secs(5));

    // Kill the ping process
    println!("Killing ping process...");
    let _ = child.kill(); // send SIGKILL

    // Wait for child to exit and collect exit status
    match child.wait() {
        Ok(status) => {
            println!("Child exited with status: {}", status);
        }
        Err(e) => {
            println!("Error waiting for child: {}", e);
        }
    }

    println!("\nNow run:");
    println!("echo $?");
    println!("…to see the exit code stored by the shell.");
}
