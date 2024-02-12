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
        if let Ok(i) = str::parse::<f64>(segment) {
            symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(i)))
        } else if let Some(c) = segment.chars().last() {
            //variables
            match c {
                'x' | 'y' => if let Ok(n) = str::parse::<f64>(&segment[..segment.len()-1]) {
                    symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(n)));
                    symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                    symbol_vec.push(match_single_char(c));
                }
                else {
                    match segment.len() {
                        1 => {
                            symbol_vec.push(match_single_char(c));
                        },
                        2 => {
                            if Some('-') == segment.chars().nth(0) {
                                symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(-1.0)));
                                symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                                symbol_vec.push(match_single_char(c));
                            }
                        },

                        _ => symbol_vec.push(match_long_operator(segment))

                    }
                },
                _ => {
                    if let Ok(n) = str::parse::<f64>(segment) {
                        symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(n)))
                    } else {
                        match segment.len() {
                            1 => {
                                symbol_vec.push(match_single_char(c));
                            },
                            _ => {
                                if Some('-') == segment.chars().nth(0) {
                                    symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(-1.0)));
                                    symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                                }
                                symbol_vec.push(match_long_operator(segment));
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

fn operator_insert(input: &mut Vec<AlgebraSymbol>, output: &mut Vec<AlgebraSymbol>, symbol: AlgebraSymbol) {
    let priority = operator_priority(&symbol);

    if let Some(priority) = priority {

        while input.last().is_some_and(|o| operator_priority(o).is_some_and(|p| p <= priority) ) &&
        !input.last().is_some_and(|s| matches!(s, AlgebraSymbol::Grouping(_))){
            if let Some(p) = input.pop() {output.push(p); }
        }
        
        input.push(symbol)

    }
}

fn pop_until_paren(operators: &mut Vec<AlgebraSymbol>, output: &mut Vec<AlgebraSymbol>) {
    while !matches!(operators.last(), Some(AlgebraSymbol::Grouping(_))) && !operators.is_empty() {
        if let Some(p) = operators.pop() {output.push(p); }
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
            AlgebraSymbol::Dual(_) => operator_insert(&mut operators, &mut postfix, symbol),
            AlgebraSymbol::Single(_) => operator_insert(&mut operators, &mut postfix, symbol),
        }

        dbg!(&operators);
    }

    pop_until_paren(&mut operators, &mut postfix);

    dbg!(&postfix);
    
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