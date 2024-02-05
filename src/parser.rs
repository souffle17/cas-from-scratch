use crate::compute::{self, NumberNode};

pub fn num_next(input: &str, x: f64) -> (f64, &str) {
    let mut out = 0.0;
    let mut negative = 0;
    if input.chars().nth(0) == Some('x') {
        (x, &input[1..])
    }
    else {
        if input.chars().nth(0) == Some('-') {
            negative = 1;
        }
        if input.chars().nth(negative).unwrap().is_ascii_digit() {
            let mut index: usize = 1; //account for space
            for char in input.chars() {
                out = out * 10.0;
                if char.is_ascii_digit() {
                    //we already checked it was a number
                    out = out + str::parse::<f64>(&char.to_string()).unwrap();
                }
                else {
                    index = index + 1;
                    break;
                }
                index = index + 1;
            }

            if negative == 1 {
                out = out * -1.0;
            }
            (out, &input[index..])
        }
        else {
            (f64::NAN, &String::new())
        }
    }
}

pub fn single_operation_match(c: char) -> Option<compute::SingleOperation> {
    match c {
        's' => Some(compute::SingleOperation::Sin),
        'c' => Some(compute::SingleOperation::Cos),
        't' => Some(compute::SingleOperation::Tan),
        'a' => Some(compute::SingleOperation::Abs),
        'q' => Some(compute::SingleOperation::Sqrt),
        _ => None
    }
}

pub fn double_operation_match(c: char) -> Option<compute::DualOperation> {
    match c {
        '+' => Some(compute::DualOperation::Plus),
        '-' => Some(compute::DualOperation::Minus),
        '*' => Some(compute::DualOperation::Multiply),
        '/' => Some(compute::DualOperation::Divide),
        'l' => Some(compute::DualOperation::Log),
        'e' => Some(compute::DualOperation::Exp),
        _ => None
    }
}

pub fn generate_single<'a>(input: &str, x: f64) -> Option<&compute::NumberNode<'a>> {
    match input.chars().nth(0).unwrap() {
        's' | 'c' | 't' | 'a' | 'q' => Some(
                &NumberNode::new(
            Some(compute::NumberOrOperation::Single(generate_single(&input[1..], x))), single_operation_match(input.chars().nth(0).unwrap())
                )
            ),
        '+' | '-' | '*' | '/' | 'l' | 'e' => Some(
            &NumberNode::new(
        Some(compute::NumberOrOperation::Double(generate_double(&input[1..], x))), double_operation_match(input.chars().nth(0).unwrap())
            )
        ),
        _ => {
            
            if num_next(&input[2..], x).0.is_nan() {
                None
            }
            else {
                let (number, new_slice) = num_next(&input[1..], x);
                match input.chars().nth(0) {
                    's' => NumberNode::new(Num)
                }
            }
        }
    }
}

pub fn generate_double<'a>(input: &str, x: f64) -> Option<&compute::NumberNode<'a>> {

}

pub fn generate<'a>(input: &str, x: f64) -> Option<&compute::NumberNode<'a>> {
    match input.chars().nth(0).unwrap() {
        's' | 'c' | 't' | 'a' | 'q' => generate_single(input, x),
        '+' | '-' | '*' | '/' | 'l' | 'e' => generate_double(input, x),
        _ => None
    }
}

pub fn point_check(left_side: &str, right_side: &str, x: f64, y: f64, err: f64) -> bool {
    let left_eq = generate(left_side, x);
    
    let right_eq = generate(right_side, x);

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