use core::fmt::{self, Formatter};

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

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equal,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    NotEqual,
}

#[derive(Debug)]
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

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Divide => write!(f, "/"),
            Self::Multiply => write!(f, "*"),
            Self::Subtract => write!(f, "-"),
        }
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "=="),
            Self::GreaterEqual => write!(f, ">="),
            Self::GreaterThan => write!(f, ">"),
            Self::LessEqual => write!(f, "<="),
            Self::LessThan => write!(f, "<"),
            Self::NotEqual => write!(f, "!="),
        }
    }
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negate => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}
