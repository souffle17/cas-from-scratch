use std::collections::VecDeque;

use crate::compute::{ConstantOrVariable, DualOperation, EquationNode, NumberNode, NumberOrOperation, SingleOperation};

fn num_next(input: &str) -> ConstantOrVariable {
    let mut negative: i64 = 1;

    let mut index_start = 0;

    if input.starts_with('_') {
        negative = -1;
        index_start = 1;
    }
    if input.chars().nth(index_start) == Some('x') {
        if negative == 1 {
            ConstantOrVariable::XVariable
        }
        else {
            ConstantOrVariable::NegativeXVariable
        }
    }
    else if input.chars().nth(index_start) == Some('y') {
        if negative == 1 {
            ConstantOrVariable::YVariable
        }
        else {
            ConstantOrVariable::NegativeYVariable
        }
    }
    else if str::parse::<f64>(&input[index_start..]).is_ok() {
        ConstantOrVariable::Constant( str::parse::<f64>(&input[index_start..]).unwrap() * negative as f64 )
    }
    else {
        ConstantOrVariable::Nan
    }
}

fn single_operation_match(c: char) -> Option<SingleOperation> {
    match c {
        's' => Some(SingleOperation::Sin),
        'c' => Some(SingleOperation::Cos),
        't' => Some(SingleOperation::Tan),
        'a' => Some(SingleOperation::Abs),
        'q' => Some(SingleOperation::Sqrt),
        _ => None
    }
}

fn double_operation_match(c: char) -> Option<DualOperation> {
    match c {
        '+' => Some(DualOperation::Plus),
        '-' => Some(DualOperation::Minus),
        '*' => Some(DualOperation::Multiply),
        '/' => Some(DualOperation::Divide),
        'l' => Some(DualOperation::Log),
        'e' => Some(DualOperation::Exp),
        _ => None
    }
}

fn generate_single(input: VecDeque<String>) -> (Option<NumberNode>, VecDeque<String>) {
    let op = single_operation_match(input.front().unwrap().chars().next().unwrap());

    let mut input = input.clone();

    input.pop_front();

    if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
        (None, input)
    }
    else {
        match input.front().unwrap().chars().next().unwrap() {
            's' | 'c' | 't' | 'a' | 'q' => {
                let (node, input) = generate_single(input);
                (Some(
                    NumberNode::new(
                        Some(Box::new(NumberOrOperation::Single(node))), op
                    )
                ), input
            )
        }
            ,
                
            '+' | '-' | '*' | '/' | 'l' | 'e' => {
                let (node, input) = generate_double(input);
                (Some(
                NumberNode::new(
                    Some(Box::new(NumberOrOperation::Double(node))), op
                )
            ), input)
        },
            _ => {

                let number = num_next(input.front().unwrap(),);

                input.pop_front();
                
                match number {
                    ConstantOrVariable::Nan => (None, input),
                    _ => {
                        (
                            Some(
                            NumberNode::new(
                                Some(Box::new(NumberOrOperation::Number(number))), None
                                )
                            ), input
                        )
                    }
                }
            }
        }
    }
}

fn generate_double(input: VecDeque<String>) -> (Option<EquationNode>, VecDeque<String>) {
    let op = double_operation_match(input.front().unwrap().chars().next().unwrap()).unwrap();

    let mut input = input.clone();

    input.pop_front();

    if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
        (None, input)
    }
    else {
        let (a, mut input) = match input.front().unwrap().chars().next().unwrap() {
            's' | 'c' | 't' | 'a' | 'q' => generate_single(input),
                
            '+' | '-' | '*' | '/' | 'l' | 'e' => generate(input),
            _ => {

                let number = num_next(input.front().unwrap());

                input.pop_front();
                
                match number {
                    ConstantOrVariable::Nan => (None, input),
                    _=> {
                        (
                            Some(
                            NumberNode::new(
                                Some(Box::new(NumberOrOperation::Number(number))), None
                                )
                            ), input
                        )
                    }
                }
            }
        };

        if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
            (None, input)
        }
        else {
            let (b, input) = match input.front().unwrap().chars().next().unwrap() {
                's' | 'c' | 't' | 'a' | 'q' => generate_single(input),
                    
                '+' | '-' | '*' | '/' | 'l' | 'e' => generate(input),

                _ => {

                    let number = num_next(input.front().unwrap());

                    input.pop_front();
                    
                    match number {
                        ConstantOrVariable::Nan => (None, input),
                        _=> {
                            (
                                Some(
                                NumberNode::new(
                                    Some(Box::new(NumberOrOperation::Number(number))), None
                                    )
                                ), input
                            )
                        }
                    }
                }
            };

            (Some(EquationNode::new(a, op, b)), input)
        }
    }
}

fn generate(input: VecDeque<String>) -> (Option<NumberNode>, VecDeque<String>) {

    if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
        (None, input)
    }
    else {
        match input.front().unwrap().chars().next().unwrap() {
            's' | 'c' | 't' | 'a' | 'q' => generate_single(input),
            '+' | '-' | '*' | '/' | 'l' | 'e' => {
                let (node, input) = generate_double(input);
                (Some(
                    NumberNode::new(
                        Some(Box::new(NumberOrOperation::Double(node))), None
                    )
                ), input)
            },
            _ => {

                let number = num_next(input.front().unwrap());

                let mut input = input.clone();

                input.pop_front();
                
                match number {
                    ConstantOrVariable::Nan => (None, input),
                    _=> {
                        (
                            Some(
                            NumberNode::new(
                                Some(Box::new(NumberOrOperation::Number(number))), None
                                )
                            ), input
                        )
                    }
                }
            }
        }
    }
}

fn split(input: &str) -> VecDeque<String> {
    let mut out: VecDeque<String> = VecDeque::new();
    let mut word: String = String::from("");

    for unit in input.chars() {
        if unit == ' '  {
            out.push_back(word.clone());
            word.clear();
        }
        else {
            word.push(unit);
        }
    }

    out.push_back(word);

    out
}

pub fn generate_tree(input: &str) -> Option<NumberNode> {

    let input_vec = &split(input);

    let (eq, _) = generate(input_vec.clone(),);

    eq
}