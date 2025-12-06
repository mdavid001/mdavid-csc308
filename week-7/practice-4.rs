use std::process::Command;

fn main(){
    let result =  Command::new("echo")
        .arg("Hello!")
        .output()
        .unwrap();

    println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
}