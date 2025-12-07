use std::fs::File;

fn main() {
    let file = File::open("hello.txt");

    // Handle the Result using match
    match file {
        Ok(_) => {
            println!("File opened successfully");
         
        }
        Err(e) => {
            println!("Failed to open the file: {}", e);
          
        }
    }
}