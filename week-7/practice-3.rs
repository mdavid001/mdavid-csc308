use std::process::Command;

fn main() {
    
    let mut child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().unwrap();
    println!("exited with :{}", status);  //status 0 means success
    
}