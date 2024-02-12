//operations that need 2 numbers
#[derive(Debug, Clone)]
pub enum DualOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Log,
    Exp
}

//operations that need 1 number
#[derive(Debug, Clone)]
pub enum SingleOperation{
    Sin,
    Cos,
    Tan,
    Abs,
    Sqrt
}

#[derive(Debug, Clone)]
pub enum NumberOrOperation {
    Double(Option<DualNode>),
    Single(Option<NumberNode>),
    Number(ConstantOrVariable)
}

#[derive(Debug, Clone)]
pub enum ConstantOrVariable {
    Constant(f64),
    XVariable,
    YVariable,
    Nan
}

// setting up the equation as a tree of operations
#[derive(Debug, Clone)]
pub struct NumberNode {
    pub value: Option<Box<NumberOrOperation>>,
    pub operation: Option<SingleOperation>
}

#[derive(Debug, Clone)]
pub struct DualNode {
    pub first_operand: Option<NumberNode>,
    pub operation: Option<DualOperation>,
    pub second_operand: Option<NumberNode>
}
fn enum_compute_dual(a: f64, b: f64, o: &Option<DualOperation>) -> f64 {
    match o {
        Some(o) => {
            match o {
                DualOperation::Plus => a + b,
                DualOperation::Minus => a - b,
                DualOperation::Divide => a / b,
                DualOperation::Multiply => a * b,
                DualOperation::Log => a.log(b),
                DualOperation::Exp => a.powf(b)
            }
        }
        None => f64::NAN
    }
}

impl NumberNode {
    // resolve the equation into a number (or not a number)
    pub fn resolve(&self, x: &f64, y: &f64) -> f64 {
        if let Some(v) = self.value.as_deref() {
            let n = match v {
                NumberOrOperation::Double (op) => {
                    if let Some(op) = op {
                        op.resolve(x, y)
                    }
                    else {
                        f64::NAN
                    }
                },
                NumberOrOperation::Single(op) => {
                    if let Some(op) = op {
                        op.resolve(x, y)
                    }
                    else {
                        f64::NAN
                    }
                }
                NumberOrOperation::Number(number) => {
                    match number {
                        ConstantOrVariable::Constant(c) => *c,
                        ConstantOrVariable::XVariable => *x,
                        ConstantOrVariable::YVariable => *y,
                        ConstantOrVariable::Nan => f64::NAN
                    }
                }
            };

            if n.is_nan() {
                f64::NAN
            }
            else if let Some(o) = self.operation.as_ref() {
                match o {
                    SingleOperation::Sin => n.sin(),
                    SingleOperation::Cos => n.cos(),
                    SingleOperation::Tan => n.tan(),
                    SingleOperation::Abs => n.abs(),
                    SingleOperation::Sqrt => n.sqrt()
                }
            }
            else {
                n
            }
        }
        else {
            f64::NAN
        }
    }

    pub fn new(value: Option<Box<NumberOrOperation>>, operation: Option<SingleOperation>) -> Self {
        Self {
            value, 
            operation
        }
    }
}

impl DualNode {
    // resolve the operation into a number (or not a number)
    fn resolve(&self, x: &f64, y: &f64) -> f64 {
        if let (Some(a), Some(b)) = (self.first_operand.as_ref(), self.second_operand.as_ref()) {
            enum_compute_dual(a.resolve(x, y), b.resolve(x, y), &self.operation)
        }
        else {
            f64::NAN
        }
    }

    pub fn new(first_operand: Option<NumberNode>, operation: Option<DualOperation>, second_operand: Option<NumberNode>) -> Self {
        Self {
            first_operand,
            operation,
            second_operand
        }
    }
}