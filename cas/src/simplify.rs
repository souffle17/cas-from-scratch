use crate::compute::NumberNode;

pub fn simplify(input_tree: Option<&NumberNode>) -> Option<NumberNode> {
    if input_tree.is_none() {
        println!("Invalid expression");
        None
    }
    else {
        let input_tree = input_tree.unwrap().clone();

        println!("Not yet implemented");

        Some(input_tree)
        
    }
}