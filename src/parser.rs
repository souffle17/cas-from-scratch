use std::collections::VecDeque;

use crate::compute::{DualOperation, EquationNode, NumberNode, NumberOrOperation, SingleOperation};

fn num_next(input: &str, x: f64, y: f64) -> f64 {
    let mut negative = 1.0;

    let mut index_start = 0;

    if input.starts_with('_') {
        negative = -1.0;
        index_start = 1;
    }
    if input.chars().nth(index_start) == Some('x') {
        x * negative
    }
    else if input.chars().nth(index_start) == Some('y') {
        y * negative
    }
    else if str::parse::<f64>(&input[index_start..]).is_ok() {
        str::parse::<f64>(&input[index_start..]).unwrap() * negative 
    }
    else {
        f64::NAN
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

fn generate_single(input: VecDeque<String>, x: f64, y: f64) -> (Option<NumberNode>, VecDeque<String>) {
    let op = single_operation_match(input.front().unwrap().chars().next().unwrap());

    let mut input = input.clone();

    input.pop_front();

    if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
        (None, input)
    }
    else {
        match input.front().unwrap().chars().next().unwrap() {
            's' | 'c' | 't' | 'a' | 'q' => {
                let (node, input) = generate_single(input, x, y);
                (Some(
                    NumberNode::new(
                        Some(Box::new(NumberOrOperation::Single(node))), op
                    )
                ), input
            )
        }
            ,
                
            '+' | '-' | '*' | '/' | 'l' | 'e' => {
                let (node, input) = generate_double(input, x, y);
                (Some(
                NumberNode::new(
                    Some(Box::new(NumberOrOperation::Double(node))), op
                )
            ), input)
        },
            _ => {

                let number = num_next(input.front().unwrap(), x, y);

                input.pop_front();
                
                if number.is_nan() {
                    (None, input)
                }
                else {
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

fn generate_double(input: VecDeque<String>, x: f64, y: f64) -> (Option<EquationNode>, VecDeque<String>) {
    let op = double_operation_match(input.front().unwrap().chars().next().unwrap()).unwrap();

    let mut input = input.clone();

    input.pop_front();

    if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
        (None, input)
    }
    else {
        let (a, mut input) = match input.front().unwrap().chars().next().unwrap() {
            's' | 'c' | 't' | 'a' | 'q' => generate_single(input, x, y),
                
            '+' | '-' | '*' | '/' | 'l' | 'e' => generate(input, x, y),
            _ => {

                let number = num_next(input.front().unwrap(), x, y);

                input.pop_front();
                
                if number.is_nan() {
                    (None, input)
                }
                else {
                    (
                        Some(
                        NumberNode::new(
                            Some(Box::new(NumberOrOperation::Number(number))), None
                            )
                        ), input
                    )
                }
            }
        };

        if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
            (None, input)
        }
        else {
            let (b, input) = match input.front().unwrap().chars().next().unwrap() {
                's' | 'c' | 't' | 'a' | 'q' => generate_single(input, x, y),
                    
                '+' | '-' | '*' | '/' | 'l' | 'e' => generate(input, x, y),

                _ => {

                    let number = num_next(input.front().unwrap(), x, y);

                    input.pop_front();
                    
                    if number.is_nan() {
                        (None, input)
                    }
                    else {
                        (
                            Some(
                            NumberNode::new(
                                Some(Box::new(NumberOrOperation::Number(number))), None
                                )
                            ), input
                        )
                    }
                }
            };

            (Some(EquationNode::new(a, op, b)), input)
        }
    }
}

fn generate(input: VecDeque<String>, x: f64, y: f64) -> (Option<NumberNode>, VecDeque<String>) {

    if input.front().is_none() || input.front().unwrap().chars().next().is_none() {
        (None, input)
    }
    else {
        match input.front().unwrap().chars().next().unwrap() {
            's' | 'c' | 't' | 'a' | 'q' => generate_single(input, x, y),
            '+' | '-' | '*' | '/' | 'l' | 'e' => {
                let (node, input) = generate_double(input, x, y);
                (Some(
                NumberNode::new(
                    Some(Box::new(NumberOrOperation::Double(node))), None
                )
            ), input)
            },
            _ => {

                let number = num_next(input.front().unwrap(), x, y);

                let mut input = input.clone();

                input.pop_front();
                
                if number.is_nan() {
                    (None, input)
                }
                else {
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

pub fn point_check(left_side: &str, right_side: &str, x: f64, y: f64, err: f64) -> bool {

    let left_eq_string_vec = &split(left_side);
    
    let right_eq_string_vec = &split(right_side);

    let (left_eq, _left) = generate(left_eq_string_vec.clone(), x, y);
    
    let (right_eq, _right) = generate(right_eq_string_vec.clone(), x, y);

    if left_eq.is_none() || right_eq.is_none() {
        false
    }
    else {
        let left = left_eq.unwrap().resolve();
        let right = right_eq.unwrap().resolve();
        if left.is_nan() || right.is_nan() {
            false
        }
        else {
            (left - right).abs() < err
        }
    }
}