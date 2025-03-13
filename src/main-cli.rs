mod collisions;
use std::env;
use std::io;
pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut n : i32;
    if args.len() > 1
    {
        n = match args[args.len()-1].to_string().parse::<i32>()
        {
            Ok(number) => number,
            Err(_) => 3
        };
    }
    else {
        println!("Enter the number of digits: ");
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        n = input_string.trim().parse().unwrap_or(3);
    }
    collisions::getcollisions(n);
}