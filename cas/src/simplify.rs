use crate::compute::{ConstantOrVariable, NumberNode, NumberOrOperation};

fn constant_to_node(constant: f64) -> Option<NumberNode> {
    Some(
        NumberNode::new(
            Some(
                Box::new(NumberOrOperation::
                    Number(ConstantOrVariable::
                        Constant(
                            constant
                        )
                    )
                )
            ), 
            None)
    )
}

fn constant_check(tree: Option<&NumberNode>) -> Option<NumberNode> {
    if tree.is_none() {
        None
    }
    else if tree.unwrap().resolve(&f64::NAN, &f64::NAN).is_nan() {
        if let Some(tree) = tree {
            
        }
        else {
            None
        }
    }
    else {
        constant_to_node(
                tree.unwrap().resolve(&f64::NAN, &f64::NAN)
        )
    }
}

pub fn simplify(input_tree: Option<&NumberNode>) -> Option<NumberNode> {
    if input_tree.is_none() {
        println!("Invalid expression");
        None
    }
    else {
        let input_tree = input_tree.unwrap().clone();

        // simplify all non-variable portions

        Some(input_tree)
        
    }
}