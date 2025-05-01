#[derive(Debug)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Comparison(ComparisonOperator),
    Logical(LogicalOperator),
}

pub trait Strength {
    fn binding_power(&self) -> (Option<u8>, Option<u8>);
}

impl Strength for Operator {
    fn binding_power(&self) -> (Option<u8>, Option<u8>) {
        match self {
            Self::Arithmetic(operator) => operator.binding_power(),
            Self::Comparison(operator) => operator.binding_power(),
            Self::Logical(operator) => operator.binding_power(),
        }
    }
}

impl From<ComparisonOperator> for Operator {
    fn from(operator: ComparisonOperator) -> Self {
        Self::Comparison(operator)
    }
}

impl From<ArithmeticOperator> for Operator {
    fn from(operator: ArithmeticOperator) -> Self {
        Self::Arithmetic(operator)
    }
}

impl From<LogicalOperator> for Operator {
    fn from(operator: LogicalOperator) -> Self {
        Self::Logical(operator)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ArithmeticOperator {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Clone, Copy, Debug)]
pub enum ComparisonOperator {
    Equal,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,
    NotEqual,
}

#[derive(Clone, Copy, Debug)]
pub enum LogicalOperator {
    Negate,
    Not,
}

impl Strength for ArithmeticOperator {
    fn binding_power(&self) -> (Option<u8>, Option<u8>) {
        match self {
            Self::Add | Self::Subtract => (Some(1), Some(2)),
            Self::Divide | Self::Multiply => (Some(3), Some(4)),
        }
    }
}

impl Strength for ComparisonOperator {
    fn binding_power(&self) -> (Option<u8>, Option<u8>) {
        (Some(5), Some(6))
    }
}

impl Strength for LogicalOperator {
    fn binding_power(&self) -> (Option<u8>, Option<u8>) {
        (None, Some(u8::MAX))
    }
}
