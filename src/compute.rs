// operations that need two numbers
#[derive(Debug)]
pub enum DualOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Log,
    Exp
}

// operations that need one number
#[derive(Debug)]
pub enum SingleOperation{
    Sin,
    Cos,
    Tan,
    Abs,
    Sqrt
}

#[derive(Debug)]
pub enum NumberOrOperation {
    Double(Option<EquationNode>),
    Single(Option<NumberNode>),
    Number(ConstantOrVariable)
}

#[derive(Debug)]
pub enum ConstantOrVariable {
    Constant(f64),
    XVariable,
    NegativeXVariable,
    YVariable,
    NegativeYVariable,
    Nan
}

// setting up the equation as a tree of operations
#[derive(Debug)]
pub struct NumberNode {
    value: Option<Box<NumberOrOperation>>,
    operation: Option<SingleOperation>
}

#[derive(Debug)]
pub struct EquationNode {
    first_operand: Option<NumberNode>,
    operation: DualOperation,
    second_operand: Option<NumberNode>
}

fn enum_compute_dual(a: f64, b: f64, o: &DualOperation) -> f64 {
    match o {
        DualOperation::Plus => a + b,
        DualOperation::Minus => a - b,
        DualOperation::Multiply => a * b,
        DualOperation::Divide => a / b,
        DualOperation::Log => a.log(b),
        DualOperation::Exp => a.powf(b)
    }
}

impl NumberNode {
    // resolve the equation into a number (or not a number)
    pub fn resolve(&self, x: f64, y: f64) -> f64 {
        match self.value {
            Some(_) => {
                let n = match self.value.as_ref().unwrap().as_ref() {
                    NumberOrOperation::Double (o) => {
                        match o {
                            Some(op) => op.resolve(x, y),
                            None => f64::NAN
                       }
                    },
                    NumberOrOperation::Single(o) => {
                        match o {
                            Some(op) => op.resolve(x, y),
                            None => f64::NAN
                        }
                    }
                    NumberOrOperation::Number(number) => {
                        match number {
                            ConstantOrVariable::Constant(c) => *c,
                            ConstantOrVariable::XVariable => x,
                            ConstantOrVariable::NegativeXVariable => -1.0 * x,
                            ConstantOrVariable::YVariable => y,
                            ConstantOrVariable::NegativeYVariable => -1.0 * y,
                            ConstantOrVariable::Nan => f64::NAN
                        }
                    }
                };

                if n.is_nan() {
                    f64::NAN
                }
                else {
                    match self.operation.as_ref() {
                        Some(o) => {
                            match o {
                                SingleOperation::Sin => n.sin(),
                                SingleOperation::Cos => n.cos(),
                                SingleOperation::Tan => n.tan(),
                                SingleOperation::Abs => n.abs(),
                                SingleOperation::Sqrt => n.sqrt()
                            }
                        }
                        None => n
                    }
                }
            }
            None => f64::NAN
        }
    }

    pub fn new(value: Option<Box<NumberOrOperation>>, operation: Option<SingleOperation>) -> Self {
        Self {
            value, 
            operation
        }
    }
}

impl EquationNode {
    // resolve the operation into a number (or not a number)
    fn resolve(&self, x: f64, y: f64) -> f64 {
        if self.first_operand.is_none() || self.second_operand.is_none() {
            f64::NAN
        }
        else {
            let a: f64 = self.first_operand.as_ref().unwrap().resolve(x, y);

            let b: f64 = self.second_operand.as_ref().unwrap().resolve(x, y);

            enum_compute_dual(a, b, &self.operation)
        }
    }

    pub fn new(first_operand: Option<NumberNode>, operation: DualOperation, second_operand: Option<NumberNode>) -> Self {
        Self {
            first_operand,
            operation,
            second_operand
        }
    }
}

pub fn point_check(left_expression: Option<&NumberNode>, right_expression: Option<&NumberNode>, x: f64, y: f64, err: f64) -> bool {

    if left_expression.is_none() || right_expression.is_none() {
        false
    }
    else {
        let left = left_expression.unwrap().resolve(x, y);
        let right = right_expression.unwrap().resolve(x, y);
        if left.is_nan() || right.is_nan() {
            false
        }
        else {
            (left - right).abs() < err
        }
    }

}