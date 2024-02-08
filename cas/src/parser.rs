use std::collections::VecDeque;

use crate::compute::{CommNode, CommOperation, ConstantOrVariable, DualNode, DualOrderOperation, NumberNode, NumberOrOperation, SingleOperation};

enum OpeningOrClosing {
    Opening,
    Closing
}

enum AlgebraSymbol {
    Grouping(OpeningOrClosing),
    Number(ConstantOrVariable),
    Variable(ConstantOrVariable),
    Dual(DualOrderOperation),
    Comm(CommOperation),
    Single(SingleOperation)
}

fn num_next(input: &str) -> ConstantOrVariable {
    let mut negative: i64 = 1;

    let mut index_start = 0;

    if input.starts_with('-') {
        negative = -1;
        index_start = 1;
    }

    let input = &input[index_start..];

    if input.starts_with('x') {
        if negative == 1 {
            ConstantOrVariable::XVariable
        }
        else {
            ConstantOrVariable::NegativeXVariable
        }
    }
    else if input.starts_with('y') {
        if negative == 1 {
            ConstantOrVariable::YVariable
        }
        else {
            ConstantOrVariable::NegativeYVariable
        }
    }
    else if str::parse::<f64>(input).is_ok() {
        ConstantOrVariable::Constant( str::parse::<f64>(input).unwrap() * negative as f64 )
    }
    else {
        ConstantOrVariable::Nan
    }
}



fn split(input: &str) -> VecDeque<String> {
    let i: VecDeque<&str> = input.split(' ').collect();
    let mut out: VecDeque<String> = VecDeque::new();
    for s in i {
        out.push_back(String::from(s));
    }

    out
}

pub fn string_to_symbol(input: &VecDeque<String>) -> VecDeque<AlgebraSymbol> {
    let mut symbol_vec: VecDeque<AlgebraSymbol> = VecDeque::new();

    for segment in input {
        match str::parse::<f64>(segment) {
            //constants
            Ok(i) => symbol_vec.push_back(AlgebraSymbol::Number(ConstantOrVariable::Constant(i))),
            Err(_) => {
                match segment.chars().last() {
                    Some(c) => {
                        //variables
                        match c {
                            'x' | 'y' => {
                                match str::parse::<f64>(&segment[..segment.len()-1]) {
                                    Ok(n) => {
                                        symbol_vec.push_back(AlgebraSymbol::Number(ConstantOrVariable::Constant(n)));
                                        symbol_vec.push_back(AlgebraSymbol::Comm(CommOperation::Multiply));
                                        match c {
                                            'x' => symbol_vec.push_back(AlgebraSymbol::Variable(ConstantOrVariable::XVariable)),
                                            'y' => symbol_vec.push_back(AlgebraSymbol::Variable(ConstantOrVariable::YVariable))
                                        }
                                    }
                                    Err(_) => {
                                        match segment.len() {
                                            1 => {
                                                match c {
                                                    'x' => symbol_vec.push_back(AlgebraSymbol::Variable(ConstantOrVariable::XVariable)),
                                                    'y' => symbol_vec.push_back(AlgebraSymbol::Variable(ConstantOrVariable::YVariable)),

                                                    //grouping symbols
                                                    '(' => symbol_vec.push_back(AlgebraSymbol::Grouping(OpeningOrClosing::Opening)),
                                                    ')' => symbol_vec.push_back(AlgebraSymbol::Grouping(OpeningOrClosing::Closing)),

                                                    //short operators
                                                    '+' => symbol_vec.push_back(AlgebraSymbol::Comm(CommOperation::Plus)),
                                                    '-' => symbol_vec.push_back(AlgebraSymbol::Dual(DualOrderOperation::Minus)),
                                                    '*' => symbol_vec.push_back(AlgebraSymbol::Comm(CommOperation::Multiply)),
                                                    '/' => symbol_vec.push_back(AlgebraSymbol::Dual(DualOrderOperation::Divide)),
                                                    '|' => symbol_vec.push_back(AlgebraSymbol::Single(SingleOperation::Abs)),
                                                    '^' => symbol_vec.push_back(AlgebraSymbol::Dual(DualOrderOperation::Exp))
                                                }
                                            },
                                            2 => {
                                                match segment.chars().next().unwrap() {
                                                    '-' => {
                                                        match c {
                                                            'x' => symbol_vec.push_back(AlgebraSymbol::Variable(ConstantOrVariable::XVariable)),
                                                            'y' => symbol_vec.push_back(AlgebraSymbol::Variable(ConstantOrVariable::YVariable))
                                                        }
                                                    }
                                                }
                                            },

                                            //longer operators
                                            _ => match segment.as_str() {
                                                "sin" => symbol_vec.push_back(AlgebraSymbol::Single(SingleOperation::Sin)),
                                                "cos" => symbol_vec.push_back(AlgebraSymbol::Single(SingleOperation::Cos)),
                                                "tan" => symbol_vec.push_back(AlgebraSymbol::Single(SingleOperation::Tan)),
                                                "sin" => symbol_vec.push_back(AlgebraSymbol::Single(SingleOperation::Sqrt)),
                                                "abs" => symbol_vec.push_back(AlgebraSymbol::Single(SingleOperation::Abs)),
                                                "log" => symbol_vec.push_back(AlgebraSymbol::Dual(DualOrderOperation::Log)),
                                            }

                                        }
                                    }
                                }
                            }
                            _ => {

                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }

    symbol_vec
    
}

pub fn generate_tree(input: VecDeque<AlgebraSymbol>) -> Option<NumberNode> {
}

pub fn generate_tree_from_string(input: &str) -> Option<NumberNode> {

    let input_vec = &split(input);

    let symbol_vec = string_to_symbol(input_vec);

    generate_tree(symbol_vec)
}