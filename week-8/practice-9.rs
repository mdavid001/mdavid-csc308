use std::fs::File;
use std::io::Write;

fn main(){
    let mut file = File::create("output.txt").unwrap();
    file.write_all(b"would inserting work?").unwrap();
//b is for byte string literal

}