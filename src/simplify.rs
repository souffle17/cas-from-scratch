use std::io::{self, Write};

use crate::parser::generate_tree;

pub fn simplify() {
    print!("Enter an expression: ");
    let _ = io::stdout().flush();

    let mut input = "".to_string();

    let _ = io::stdin().read_line(&mut input).is_ok();

    input = input.trim_end().to_string();

    let input_tree = generate_tree(&input);
    
    if input_tree.is_none() {
        println!("Invalid expression");
    }
    else {
        let input_tree = input_tree.unwrap();

        todo!();
        
    }
}