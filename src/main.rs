use std::io::{self, Write};

mod compute;
mod parser;
mod draw;
mod simplify;

fn main() {
    let mut input: String = "".to_string();
    loop {
        input.clear();
        println!("0. Quit");
        println!("1. Graph an equation");
        println!("2. Simplify an expression");
        print!("Pick an operation by number: ");
        let _ = io::stdout().flush();


        let _ = io::stdin().read_line(&mut input).is_ok();

        input = input.trim_end().to_string();

        match str::parse::<i32>(&input) {
            Ok(i) => {
                match i {
                    0 => break,
                    1 => draw::draw(),
                    2 => simplify::simplify(),
                    _ => println!("Invalid operation")
                }
            }
            Err(_) => println!("Invalid operation")
        }
        
    }
    
}