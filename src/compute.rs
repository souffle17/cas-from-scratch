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

pub enum NumberOrOperation<'a> {
    Double(Option<&'a EquationNode<'a>>),
    Single(Option<&'a NumberNode<'a>>),
    Number(f64)
}

// setting up the equation as a tree of operations
pub struct NumberNode<'a> {
    value: Option<NumberOrOperation<'a>>,
    operation: Option<SingleOperation>
}

pub struct EquationNode<'a> {
    first_operand: NumberNode<'a>,
    operation: DualOperation,
    second_operand: NumberNode<'a>
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

impl NumberNode<'_> {
    // resolve the equation into a number (or not a number)
    pub fn resolve(&self) -> f64 {
        match self.value {
            Some(_) => {
                let x: f64 = match self.value.unwrap() {
                    NumberOrOperation::Double(o) => {
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
                    NumberOrOperation::Number(n) => n
                };

                if x.is_nan() {
                    f64::NAN
                }
                else {
                    match &self.operation {
                        Sin => x.sin(),
                        Cos => x.cos(),
                        Tan => x.tan(),
                        Ln => x.ln(),
                        Abs => x.abs(),
                        Sqrt => x.sqrt(),
                        _ => x
                    }
                }
            }
            None => f64::NAN
        }
    }

    pub fn new(value: Option<NumberOrOperation>, operation: Option<SingleOperation>) -> Self {
        Self {
            value, 
            operation
        }
    }
}

impl EquationNode<'_> {
    // resolve the operation into a number (or not a number)
    fn resolve(&self) -> f64 {

        let a: f64 = self.first_operand.resolve();

        let b: f64 = self.second_operand.resolve();

        enum_compute_dual(a, b, &self.operation)
    }

    pub fn new(first_operand: NumberNode<'_>, operation: DualOperation, second_operand: NumberNode<'_>) -> Self {
        Self {
            first_operand,
            operation,
            second_operand
        }
    }
}