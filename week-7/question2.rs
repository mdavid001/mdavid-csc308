use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let result = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .expect("Failed to run echo command");

    //  bytes to a string
    let stdout_text = String::from_utf8_lossy(&result.stdout);

    // Write the output to output.txt
    let mut file = File::create("output.txt")
        .expect("Failed to create file");

    file.write_all(stdout_text.as_bytes())
        .expect("Failed to write to file");

    println!("Output written to output.txt");
}
