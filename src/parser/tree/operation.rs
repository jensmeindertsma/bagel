use super::{ArithmeticOperator, ComparisonOperator, LogicalOperator, Tree};
use core::fmt::{self, Formatter};

#[derive(Debug)]
pub enum Operation {
    Arithmetic {
        operator: ArithmeticOperator,
        a: Box<Tree>,
        b: Box<Tree>,
    },
    Comparison {
        operator: ComparisonOperator,
        a: Box<Tree>,
        b: Box<Tree>,
    },
    Group(Box<Tree>),
    Logical {
        operator: LogicalOperator,
        expression: Box<Tree>,
    },
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arithmetic { operator, a, b } => write!(f, "({operator} {a} {b})"),
            Self::Comparison { operator, a, b } => write!(f, "({operator} {a} {b})"),
            Self::Group(expression) => write!(f, "(group {expression})"),
            Self::Logical {
                operator,
                expression,
            } => write!(f, "({operator} {expression})"),
        }
    }
}
