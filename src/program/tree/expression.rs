pub mod operator;

use operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator};

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub line: usize,
}

impl Expression {
    pub fn operation(operation: Operation, line: usize) -> Self {
        Self {
            kind: ExpressionKind::Operation(operation),
            line,
        }
    }

    pub fn primitive(primitive: Primitive, line: usize) -> Self {
        Self {
            kind: ExpressionKind::Primitive(primitive),
            line,
        }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Operation(Operation),
    Primitive(Primitive),
}

#[derive(Debug)]
pub enum Operation {
    Arithmetic {
        operator: ArithmeticOperator,
        a: Box<Expression>,
        b: Box<Expression>,
    },
    Comparison {
        operator: ComparisonOperator,
        a: Box<Expression>,
        b: Box<Expression>,
    },
    Group(Box<Expression>),
    Logical {
        operator: LogicalOperator,
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Primitive {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}
