mod operation;
mod operator;
mod primitive;

pub use operation::Operation;
pub use operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator, Operator, Strength};
pub use primitive::Primitive;

use core::fmt::{self, Formatter};

#[derive(Debug)]
pub enum Tree {
    Operation(Operation),
    Primitive(Primitive),
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation(operation) => write!(f, "{operation}"),
            Self::Primitive(primitive) => write!(f, "{primitive}"),
        }
    }
}
