use crate::compute::{ConstantOrVariable, DualNode, DualOperation, NumberNode, NumberOrOperation, SingleOperation};

#[derive(Debug, Clone)]
pub enum OpeningOrClosing {
    Opening,
    Closing
}

#[derive(Debug, Clone)]
pub enum AlgebraSymbol {
    Grouping(OpeningOrClosing),
    Number(ConstantOrVariable),
    Variable(ConstantOrVariable),
    Dual(DualOperation),
    Single(SingleOperation)
}


pub fn string_to_symbol(input: &Vec<String>) -> Vec<AlgebraSymbol> {
    let mut symbol_vec: Vec<AlgebraSymbol> = Vec::new();

    for segment in input {
        match str::parse::<f64>(segment) {
            //constants
            Ok(i) => symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(i))),
            Err(_) => {
                if let Some(c) = segment.chars().last() {
                    //variables
                    match c {
                        'x' | 'y' => {
                            match str::parse::<f64>(&segment[..segment.len()-1]) {
                                Ok(n) => {
                                    symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(n)));
                                    symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                                    match c {
                                        'x' => symbol_vec.push(AlgebraSymbol::Variable(ConstantOrVariable::XVariable)),
                                        'y' => symbol_vec.push(AlgebraSymbol::Variable(ConstantOrVariable::YVariable)),
                                        _ => {}
                                    }
                                }
                                Err(_) => {
                                    match segment.len() {
                                        1 => {
                                            match c {
                                                'x' => symbol_vec.push(AlgebraSymbol::Variable(ConstantOrVariable::XVariable)),
                                                'y' => symbol_vec.push(AlgebraSymbol::Variable(ConstantOrVariable::YVariable)),

                                                //grouping symbols
                                                '(' => symbol_vec.push(AlgebraSymbol::Grouping(OpeningOrClosing::Opening)),
                                                ')' => symbol_vec.push(AlgebraSymbol::Grouping(OpeningOrClosing::Closing)),

                                                //short operators
                                                '+' => symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Plus)),
                                                '-' => symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Minus)),
                                                '*' => symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply)),
                                                '/' => symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Divide)),
                                                '^' => symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Exp)),
                                                _ => {}
                                            }
                                        },
                                        2 => {
                                            if let '-' = segment.chars().next().unwrap() {
                                                symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(-1.0)));
                                                symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                                                match c {
                                                    'x' => symbol_vec.push(AlgebraSymbol::Variable(ConstantOrVariable::XVariable)),
                                                    'y' => symbol_vec.push(AlgebraSymbol::Variable(ConstantOrVariable::YVariable)),
                                                    _ => {}
                                                }
                                            }
                                        },

                                        //longer operators
                                        _ => match segment.as_str() {
                                            "sin" => symbol_vec.push(AlgebraSymbol::Single(SingleOperation::Sin)),
                                            "cos" => symbol_vec.push(AlgebraSymbol::Single(SingleOperation::Cos)),
                                            "tan" => symbol_vec.push(AlgebraSymbol::Single(SingleOperation::Tan)),
                                            "sqrt" => symbol_vec.push(AlgebraSymbol::Single(SingleOperation::Sqrt)),
                                            "abs" => symbol_vec.push(AlgebraSymbol::Single(SingleOperation::Abs)),
                                            "log" => symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Log)),
                                            _ => {}
                                        }

                                    }
                                }
                            }
                        }
                        _ => {

                        }
                    }
                }
            }
        }
    }

    symbol_vec
    
}

fn operator_priority(symbol: &AlgebraSymbol) -> Option<i32> {
    match symbol {
        AlgebraSymbol::Single(_) => Some(0),
        AlgebraSymbol::Dual(op) => {
            match op {
                DualOperation::Exp | DualOperation::Log => Some(1),
                DualOperation::Multiply | DualOperation::Divide => Some(2),
                DualOperation::Plus | DualOperation::Minus => Some(3),
            }
        }
        _ => None //this shouldn't be reached
    }
}

fn operator_insert(input: &mut Vec<AlgebraSymbol>, symbol: AlgebraSymbol) {
    let priority = operator_priority(&symbol);

    if priority.is_none() {
        return;
    }

    let priority = priority.unwrap();

    for index in (0..input.len()).rev() {
        if operator_priority(&input[index]) >= Some(priority) || 
            index == 0 || 
                matches!(input[index-1], AlgebraSymbol::Grouping(_)) {
            
            input.insert(index+1, symbol.clone());
            break;
        }
    }
}

fn pop_until_paren(operators: &mut Vec<AlgebraSymbol>, output: &mut Vec<AlgebraSymbol>) {
    while !matches!(operators.first(), Some(AlgebraSymbol::Grouping(_))) && !operators.is_empty() {
        output.push(operators.pop().unwrap());
    }

    if matches!(operators.first(), Some(AlgebraSymbol::Grouping(_))) {
        operators.pop();
    }
}

pub fn generate_tree(input: Vec<AlgebraSymbol>) -> Option<NumberNode> {
    //https://en.wikipedia.org/wiki/Shunting_yard_algorithm

    let mut operators: Vec<AlgebraSymbol> = Vec::new();

    let mut postfix: Vec<AlgebraSymbol> = Vec::new();

    for symbol in input {
        match symbol {
            AlgebraSymbol::Grouping(OpeningOrClosing::Opening) => operators.push(symbol),
            AlgebraSymbol::Grouping(OpeningOrClosing::Closing) => pop_until_paren(&mut operators, &mut postfix),
            AlgebraSymbol::Number(_) => postfix.push(symbol),
            AlgebraSymbol::Variable(_) => postfix.push(symbol),
            AlgebraSymbol::Dual(_) => operator_insert(&mut operators, symbol),
            AlgebraSymbol::Single(_) => operator_insert(&mut operators, symbol),
        }
    }

    pop_until_paren(&mut operators, &mut postfix);

    postfix.reverse();

    assemble_tree(postfix).0
}

fn assemble_dual(input: Vec<AlgebraSymbol>) -> (Option<DualNode>, Vec<AlgebraSymbol>) {
    let mut input = input.clone();

    let operator: Option<DualOperation> = match input.pop() {
        Some(symbol) => {
            if let AlgebraSymbol::Dual(op) = symbol {
                Some(op)
            } else {
                None
            }
        }
        None => None
    };


    let first_operand;
    let second_operand;

    (first_operand, input) = assemble_tree(input);
    (second_operand, input) = assemble_tree(input);

    (Some(DualNode::new(first_operand, operator, second_operand)), input)
}

fn assemble_single(input: Vec<AlgebraSymbol>) -> (Option<NumberNode>, Vec<AlgebraSymbol>) {
    let mut input = input.clone();
    let operator: Option<SingleOperation> = match input.pop() {
        Some(symbol) => {
            if let AlgebraSymbol::Single(op) = symbol {
                Some(op)
            } else {
                None
            }
        },
        None => None
    };

    let node;

    (node, input) = assemble_tree(input);

    (Some(NumberNode::new(Some(Box::new(NumberOrOperation::Single(node))), operator)), input)
}

fn assemble_tree(input: Vec<AlgebraSymbol>) -> (Option<NumberNode>, Vec<AlgebraSymbol>) {
    let mut input = input.clone();

    if input.first().is_none() {
        return (None, Vec::new());
    }

    let node = input.first().unwrap().clone();
    match node {
        AlgebraSymbol::Single(_) => assemble_single(input),
        AlgebraSymbol::Dual(_) => {
            let node;
            (node, input) = assemble_dual(input);
            (Some(NumberNode::new(Some(Box::new(NumberOrOperation::Double(node))), None)), input)
        },
        AlgebraSymbol::Number(n) | 
        AlgebraSymbol::Variable(n) => {
            input.pop();
            (Some(NumberNode::new(Some(Box::new(NumberOrOperation::Number(n))), None)), input)
        }
        _ => (None, Vec::new())
    }
}

fn split(input: &str) -> Vec<String> {
    let i: Vec<&str> = input.split(' ').collect();
    let mut out: Vec<String> = Vec::new();
    for s in i {
        out.push(String::from(s));
    }

    out
}

pub fn generate_tree_from_string(input: &str) -> Option<NumberNode> {

    let input_vec = &split(input);

    let symbol_vec = string_to_symbol(input_vec);

    generate_tree(symbol_vec)
}