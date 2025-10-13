fn main() {
    let mut count = 0;
    loop {
        println!("count: {}", count);
        count +=1;
        if count == 3 {break;}
    }

    let mut n = 0;
    while n < 5 {
        println!("{}",n);
        n += 1;
    }

    for i in 1..=5{
        println!("number: {}", i); 
    }
}