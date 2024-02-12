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

fn match_operator(input: &str) -> AlgebraSymbol {
    match input {
        //long operators
        "sin" => AlgebraSymbol::Single(SingleOperation::Sin),
        "cos" => AlgebraSymbol::Single(SingleOperation::Cos),
        "tan" => AlgebraSymbol::Single(SingleOperation::Tan),
        "sqrt" => AlgebraSymbol::Single(SingleOperation::Sqrt),
        "abs" => AlgebraSymbol::Single(SingleOperation::Abs),
        "log" => AlgebraSymbol::Dual(DualOperation::Log),

        //variables
        "x" => AlgebraSymbol::Variable(ConstantOrVariable::XVariable),
        "y" => AlgebraSymbol::Variable(ConstantOrVariable::YVariable),

        //grouping symbols
        "(" => AlgebraSymbol::Grouping(OpeningOrClosing::Opening),
        ")" => AlgebraSymbol::Grouping(OpeningOrClosing::Closing),

        //short operators
        "+" => AlgebraSymbol::Dual(DualOperation::Plus),
        "-" => AlgebraSymbol::Dual(DualOperation::Minus),
        "*" => AlgebraSymbol::Dual(DualOperation::Multiply),
        "/" => AlgebraSymbol::Dual(DualOperation::Divide),
        "^" => AlgebraSymbol::Dual(DualOperation::Exp),
        _ => AlgebraSymbol::Number(ConstantOrVariable::Nan)
        
    }
}

fn symbol_to_string(input: AlgebraSymbol) -> String {
    match input {
        AlgebraSymbol::Single(op) => {
            match op {
                SingleOperation::Sin => "sin".to_string(),
                SingleOperation::Cos => "cos".to_string(),
                SingleOperation::Tan => "tan".to_string(),
                SingleOperation::Abs => "abs".to_string(),
                SingleOperation::Sqrt => "sqrt".to_string()
            }
        }

        AlgebraSymbol::Grouping(paren) => {
            match paren {
                OpeningOrClosing::Opening => "(".to_string(),
                OpeningOrClosing::Closing => ")".to_string()
            }
        }

        AlgebraSymbol::Dual(dual) => {
            match dual {
                DualOperation::Plus => "+".to_string(),
                DualOperation::Minus => "-".to_string(),
                DualOperation::Multiply => "*".to_string(),
                DualOperation::Divide => "/".to_string(),
                DualOperation::Exp => "^".to_string(),
                DualOperation::Log => "log".to_string()
            }
        }

        AlgebraSymbol::Number(n) | AlgebraSymbol::Variable(n) => {
            match n {
                ConstantOrVariable::Constant(n) => n.to_string(),
                ConstantOrVariable::XVariable => "x".to_string(),
                ConstantOrVariable::YVariable => "y".to_string(),
                ConstantOrVariable::Nan => "Nan".to_string()
            }
        }
    }
}

pub fn string_vec_to_symbol_vec(input: &Vec<String>) -> Vec<AlgebraSymbol> {
    let mut symbol_vec: Vec<AlgebraSymbol> = Vec::new();

    for segment in input {
        if let Ok(i) = str::parse::<f64>(segment) {
            symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(i)))
        } else if let Some(c) = segment.chars().last() {
            match c {
                //variables
                'x' | 'y' => if let Ok(n) = str::parse::<f64>(&segment[..segment.len()-1]) {
                        symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(n)));
                        symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                        symbol_vec.push(match_operator(&c.to_string()));
                    }
                    else {
                        if segment.starts_with('-') {
                            symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(-1.0)));
                            symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                        }
                        symbol_vec.push(match_operator(&c.to_string()));
                    },
                _ => {
                    if let Ok(n) = str::parse::<f64>(segment) {
                        symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(n)))
                    } else if segment.starts_with('-') && segment.len() > 1 {
                        symbol_vec.push(AlgebraSymbol::Number(ConstantOrVariable::Constant(-1.0)));
                        symbol_vec.push(AlgebraSymbol::Dual(DualOperation::Multiply));
                        symbol_vec.push(match_operator(&segment[1..segment.len()]));
                    }
                    else {
                        symbol_vec.push(match_operator(segment));
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

        while input.last().is_some_and(|o| 
            operator_priority(o).is_some_and(
                |p| p <= priority) ) &&
                    !input.last().is_some_and(
                        |s| matches!(s, AlgebraSymbol::Grouping(_))) {
            if let Some(p) = input.pop() {
                output.push(p);
            }
        }
        
        input.push(symbol)

    }
}

fn pop_until_paren(operators: &mut Vec<AlgebraSymbol>, output: &mut Vec<AlgebraSymbol>) {
    while !matches!(operators.last(), Some(AlgebraSymbol::Grouping(_))) && !operators.is_empty() {
        if let Some(p) = operators.pop() {
            output.push(p);
        }
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

    let symbol_vec = string_vec_to_symbol_vec(input_vec);

    generate_tree(symbol_vec)
}

pub fn expression_to_string(input: Option<NumberNode>) -> String {
    let mut output = " ".to_string();

    if let Some(input) = input {
        if let Some(v) = input.value {
            match *v {
                NumberOrOperation::Double(dual) => {
                    if let Some(dual) = dual {
                        if let Some(first) = dual.first_operand {
                            output.push_str(&expression_to_string(Some(first)));
                        }
                        output.push(' ');
                        if let Some(op) = dual.operation {
                            let symbol = AlgebraSymbol::Dual(op);
                            output.push_str(&symbol_to_string(symbol));
                        }
                        output.push(' ');
                        if let Some(second) = dual.second_operand {
                            output.push_str(&expression_to_string(Some(second)));
                        }
                    }
                },
                NumberOrOperation::Single(opt_node) => {
                    if let Some(node) = opt_node {
                        if let Some(op) = node.operation.clone() {
                            let symbol = AlgebraSymbol::Single(op);
                            output.push_str(&symbol_to_string(symbol));
                            output.push(' ');
                            output.push_str(&expression_to_string(Some(node)));
                        }
                    }
                },
                NumberOrOperation::Number(n) => {
                    let symbol = AlgebraSymbol::Number(n);
                    output.push_str(&symbol_to_string(symbol));
                },
            }
        }
    }

    output
}