//commutative operations
#[derive(Debug)]
pub enum CommOperation {
    Plus,
    Multiply
}

//operations that need 2 numbers
#[derive(Debug)]
pub enum DualOrderOperation {
    Minus,
    Divide,
    Log,
    Exp
}

//operations that need 1 number
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
    Comm(Option<CommNode>),
    Double(Option<DualNode>),
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
pub struct DualNode {
    first_operand: Option<NumberNode>,
    operation: DualOrderOperation,
    second_operand: Option<NumberNode>
}

#[derive(Debug)]
pub struct CommNode {
    numbers: Vec<NumberNode>,
    operation: CommOperation
}

fn enum_compute_dual(a: f64, b: f64, o: &DualOrderOperation) -> f64 {
    match o {
        DualOrderOperation::Minus => a - b,
        DualOrderOperation::Divide => a / b,
        DualOrderOperation::Log => a.log(b),
        DualOrderOperation::Exp => a.powf(b)
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
                            ConstantOrVariable::NegativeXVariable => -1.0 * x,
                            ConstantOrVariable::YVariable => *y,
                            ConstantOrVariable::NegativeYVariable => -1.0 * y,
                            ConstantOrVariable::Nan => f64::NAN
                        }
                    }
                    NumberOrOperation::Comm(comm) => {
                        match comm {
                            Some(comm) => comm.resolve(x, y),
                            None => f64::NAN
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

    pub fn new(first_operand: Option<NumberNode>, operation: DualOrderOperation, second_operand: Option<NumberNode>) -> Self {
        Self {
            first_operand,
            operation,
            second_operand
        }
    }
}

impl CommNode {
    
    // resolve the operation into a number (or not a number)
    fn resolve(&self, x: &f64, y: &f64) -> f64 {

        let mut total = 0.0;

        match self.operation {
            CommOperation::Multiply => {
                for n in self.numbers {
                    total *= n.resolve(&x, &y)
                }
            }
            CommOperation::Plus => {
                for n in self.numbers {
                    total += n.resolve(&x, &y)
                }
            }
        }

        total
    }
}