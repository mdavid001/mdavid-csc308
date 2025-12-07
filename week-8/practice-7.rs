use std::fs;

fn main(){
    let content =fs::read_to_string("hello.txt")
        .expect("couldnt read file");
    println!("{}", content);
}