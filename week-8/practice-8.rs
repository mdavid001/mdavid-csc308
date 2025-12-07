use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("hello.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {  
    println!("{}", line.unwrap());
    }
       
}