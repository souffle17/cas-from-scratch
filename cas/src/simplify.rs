use crate::compute::{ConstantOrVariable, NumberNode, NumberOrOperation};

fn constant_to_node(constant: f64) -> Option<NumberNode> {
    Some(NumberNode {
        value: Some(Box::new(NumberOrOperation::Number(
            ConstantOrVariable::Constant(constant),
        ))),
        operation: None,
    })
}

fn constant_check(tree: Option<NumberNode>) -> Option<NumberNode> {
    if let Some(mut tree) = tree.clone() {
        if tree.resolve(&f64::NAN, &f64::NAN).is_nan() {
            if let Some(boxed_value) = tree.value.clone() {
                match *boxed_value {
                    NumberOrOperation::Double(dual) => {
                        if let Some(mut dual) = dual {
                            dual.first_operand = constant_check(dual.first_operand);
                            dual.second_operand = constant_check(dual.second_operand);
                            tree.value = Some(Box::new(NumberOrOperation::Double(Some(dual))));

                            Some(tree.clone())
                        } else {
                            None
                        }
                    }
                    NumberOrOperation::Single(node) => {
                        tree.value =
                            Some(Box::new(NumberOrOperation::Single(constant_check(node))));
                        Some(tree.clone())
                    }
                    NumberOrOperation::Number(_) => Some(tree),
                }
            } else {
                None
            }
        } else {
            constant_to_node(tree.resolve(&f64::NAN, &f64::NAN))
        }
    } else {
        None
    }
}

pub fn simplify(input_tree: Option<NumberNode>) -> Option<NumberNode> {
    let mut input_tree = input_tree.clone();

    // simplify all constants

    input_tree = constant_check(input_tree);

    input_tree
}
