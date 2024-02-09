use core::num;

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

fn match_single_char(c: char) -> AlgebraSymbol {
    match c {
        'x' => AlgebraSymbol::Variable(ConstantOrVariable::XVariable),
        'y' => AlgebraSymbol::Variable(ConstantOrVariable::YVariable),

        //grouping symbols
        '(' => AlgebraSymbol::Grouping(OpeningOrClosing::Opening),
        ')' => AlgebraSymbol::Grouping(OpeningOrClosing::Closing),

        //short operators
        '+' => AlgebraSymbol::Dual(DualOperation::Plus),
        '-' => AlgebraSymbol::Dual(DualOperation::Minus),
        '*' => AlgebraSymbol::Dual(DualOperation::Multiply),
        '/' => AlgebraSymbol::Dual(DualOperation::Divide),
        '^' => AlgebraSymbol::Dual(DualOperation::Exp),
        _ => AlgebraSymbol::Number(ConstantOrVariable::Nan)
    }
}

fn match_long_operator(input: &str) -> AlgebraSymbol {
    match input {
        "sin" => AlgebraSymbol::Single(SingleOperation::Sin),
        "cos" => AlgebraSymbol::Single(SingleOperation::Cos),
        "tan" => AlgebraSymbol::Single(SingleOperation::Tan),
        "sqrt" => AlgebraSymbol::Single(SingleOperation::Sqrt),
        "abs" => AlgebraSymbol::Single(SingleOperation::Abs),
        "log" => AlgebraSymbol::Dual(DualOperation::Log),
        _ => AlgebraSymbol::Number(ConstantOrVariable::Nan)
    }
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
                                    symbol_vec.push(match_single_char(c));
                                }
                                Err(_) => {
                                    match segment.len() {
                                        1 => {
                                            symbol_vec.push(match_single_char(c));
                                        },
                                        2 => {
                                            if let '-' = segment.chars().next().unwrap() {
                                                symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(-1.0)));
                                                symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                                                symbol_vec.push(match_single_char(c));
                                            }
                                        },

                                        _ => symbol_vec.push(match_long_operator(segment))

                                    }
                                }
                            }
                        }
                        _ => {
                            match str::parse::<f64>(segment) {
                                Ok(n) => symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(n))),
                                Err(_) => {
                                    match segment.len() {
                                        1 => {
                                            symbol_vec.push(match_single_char(c));
                                        },
                                        _ => symbol_vec.push(match_long_operator(segment))
                                    }
                                }
                            }
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
        let temp_priority = operator_priority(&input[index]).unwrap_or(-1);
        if temp_priority >= priority || 
                matches!(input[index], AlgebraSymbol::Grouping(_)) {
            input.insert(index+1, symbol.clone());
            return;
        }
    }
    if input.is_empty() {
        input.insert(0, symbol);
    }
}

fn pop_until_paren(operators: &mut Vec<AlgebraSymbol>, output: &mut Vec<AlgebraSymbol>) {
    while !matches!(operators.last(), Some(AlgebraSymbol::Grouping(_))) && !operators.is_empty() {
        output.push(operators.pop().unwrap());
    }

    if matches!(operators.last(), Some(AlgebraSymbol::Grouping(_))) {
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
    
    let mut numbers: Vec<NumberNode> = Vec::new();

    for symbol in postfix {
        match symbol {
            AlgebraSymbol::Number(n) => numbers.push(NumberNode::new(Some(Box::new(NumberOrOperation::Number(n))), None)),
            AlgebraSymbol::Variable(x) => numbers.push(NumberNode::new(Some(Box::new(NumberOrOperation::Number(x))), None)),
            AlgebraSymbol::Dual(op) => {
                let second_operand = numbers.pop();
                let first_operand = numbers.pop();

                let dual_node = DualNode::new(first_operand, Some(op), second_operand);
                numbers.push(
                    NumberNode::new(Some(Box::new(NumberOrOperation::Double(Some(dual_node)))), None)
                );
            }
            AlgebraSymbol::Single(op) => {
                let new_node = NumberNode::new(Some(Box::new(NumberOrOperation::Single(numbers.pop()))), Some(op));
                numbers.push(new_node);
            }
            _ => {}
        }
    }

    numbers.pop()
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