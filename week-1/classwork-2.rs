use std::io;

fn main() {
    println!("Welcome to Smart Cafe.");
    println!("How much did you spend? : ");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();
    let ans_input: f64 = ans.trim().parse().unwrap();

    if ans_input > 10000.0{
        println!("Original bill: {}",ans_input);
        println!("Discount applied: 15%");
        println!("Final bill: {}", ans_input*0.85);
    }
    else if ans_input > 5000.0{
        println!("Original bill: {}",ans_input);
        println!("Discount applied: 10%");
        println!("Final bill: {}", ans_input*0.9);
    }
    else{
        println!("Your total bill is {}. No discount was applicable.", ans_input);
    }
}