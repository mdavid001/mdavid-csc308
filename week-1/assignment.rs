use std::io;

fn main() {
    println!("EKEDC Smart Meter");
    println!("How many units(kwh) do you use?");

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();
    let units: f64 = ans.trim().parse().unwrap();

    if units > 200.0{
        println!("Total electricity bill: {} naira", units * 30.0);
    }
    else if units > 100.0{
        println!("Total electricity bill: {} naira", units * 25.0);
    }
    else{
        println!("Total electricity bill: {} naira", units * 20.0);
    }
}