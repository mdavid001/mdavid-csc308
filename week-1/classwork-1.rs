use std::io;

fn main(){
    loop{
        println!("Enter 1 to convert from fahrenheit to Celsius");
        println!("Enter 2 to convert from Celsius to fahrenheit \n:");

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        let ans_input: i32 = ans.trim().parse().unwrap();

        if ans_input == 1{
            println!("Enter your temperature in Fahrenheit: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let user_input: f64 = input.trim().parse().unwrap();
            let degree = (user_input - 32.0) * (5.0/9.0);
            println!("converted temperature: {:.2}C", degree);
            break;
        }
        else if ans_input ==2{
            println!("Enter your temperature in Celsius: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let user_input: f64 = input.trim().parse().unwrap();
            let degree = (user_input * (9.0/5.0)) + 32.0;
            println!("converted temperature: {:.2}F", degree);
            break;
        }
        else{
            println!("please pick between available options")
        }
    }


}