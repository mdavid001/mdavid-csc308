use std::fs::OpenOptions;
use std::io::Write;

fn main(){
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    writeln!(file,"new log line").unwrap();
}