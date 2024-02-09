//commutative operations
#[derive(Debug)]
pub enum CommOperation {
}

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
        match self.value {
            Some(_) => {
                let n = match self.value.as_ref().unwrap().as_ref() {
                    NumberOrOperation::Double (op) => {
                        match op {
                            Some(op) => op.resolve(x, y),
                            None => f64::NAN
                       }
                    },
                    NumberOrOperation::Single(op) => {
                        match op {
                            Some(op) => op.resolve(x, y),
                            None => f64::NAN
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

impl DualNode {
    // resolve the operation into a number (or not a number)
    fn resolve(&self, x: &f64, y: &f64) -> f64 {
        if self.first_operand.is_none() || self.second_operand.is_none() {
            f64::NAN
        }
        else {
            let a: f64 = self.first_operand.as_ref().unwrap().resolve(x, y);

            let b: f64 = self.second_operand.as_ref().unwrap().resolve(x, y);

            enum_compute_dual(a, b, &self.operation)
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