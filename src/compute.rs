// operations that need two numbers
pub enum DualOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Log,
    Exp
}

// operations that need one number
pub enum SingleOperation{
    Sin,
    Cos,
    Tan,
    Abs,
    Sqrt
}

pub enum NumberOrOperation {
    Double(Option<EquationNode>),
    Single(Option<NumberNode>),
    Number(f64)
}

// setting up the equation as a tree of operations
pub struct NumberNode {
    value: Option<Box<NumberOrOperation>>,
    operation: Option<SingleOperation>
}

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
    pub fn resolve(&self) -> f64 {
        match self.value {
            Some(_) => {
                let x: f64 = match self.value.as_ref().unwrap().as_ref() {
                    NumberOrOperation::Double (o) => {
                        match o {
                            Some(op) => op.resolve(),
                            None => f64::NAN
                        }
                    },
                    NumberOrOperation::Single(o) => {
                        match o {
                            Some(op) => op.resolve(),
                            None => f64::NAN
                        }
                    }
                    NumberOrOperation::Number(n) => *n
                };

                if x.is_nan() {
                    f64::NAN
                }
                else {
                    match self.operation.as_ref() {
                        Some(o) => {
                            match o {
                                SingleOperation::Sin => x.sin(),
                                SingleOperation::Cos => x.cos(),
                                SingleOperation::Tan => x.tan(),
                                SingleOperation::Abs => x.abs(),
                                SingleOperation::Sqrt => x.sqrt()
                            }
                        }
                        None => x
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
    fn resolve(&self) -> f64 {
        if self.first_operand.is_none() || self.second_operand.is_none() {
            f64::NAN
        }
        else {
            let a: f64 = self.first_operand.as_ref().unwrap().resolve();

            let b: f64 = self.second_operand.as_ref().unwrap().resolve();

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