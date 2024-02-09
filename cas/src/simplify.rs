use crate::{compute::NumberNode};

pub fn simplify(input_tree: Option<NumberNode>) {
    if input_tree.is_none() {
        println!("Invalid expression");
    }
    else {
        let _input_tree = input_tree.unwrap();

        todo!();
        
    }
}