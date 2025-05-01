use super::{
    operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator},
    Tree,
};

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
