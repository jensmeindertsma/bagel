use super::tree::{
    operation::Operation, operator::ArithmeticOperator, primitive::Primitive, visitor::Visitor,
    Tree,
};
use std::{
    error::Error,
    fmt::{self, Formatter},
};

pub struct Interpreter {
    tree: Tree,
}

impl Interpreter {
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }

    pub fn evaluate(&mut self) -> Result<Value, InterpreterError> {
        self.visit_tree(&self.tree)
    }

    fn visit_tree(&self, tree: &Tree) -> Result<Value, InterpreterError> {
        match tree {
            Tree::Operation(operation) => self.visit_operation(operation),
            Tree::Primitive(primitive) => self.visit_primitive(primitive),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(value) => {
                if value.fract() == 0.0 {
                    write!(f, "{}", value.trunc())
                } else {
                    write!(f, "{value}")
                }
            }
            Self::String(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    InvalidAddition(Value, Value),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAddition(a, b) => {
                write!(f, "invalid addition of {a} and {b}")
            }
        }
    }
}

impl Error for InterpreterError {}

impl Visitor<Result<Value, InterpreterError>> for Interpreter {
    fn visit_operation(
        &self,
        operation: &super::tree::operation::Operation,
    ) -> Result<Value, InterpreterError> {
        match operation {
            Operation::Arithmetic { operator, a, b } => {
                let a = self.visit_tree(a)?;
                let b = self.visit_tree(b)?;
                match (&a, &b, operator) {
                    (Value::Number(a), Value::Number(b), ArithmeticOperator::Add) => {
                        Ok(Value::Number(a + b))
                    }
                    (Value::Number(a), Value::Number(b), ArithmeticOperator::Divide) => {
                        Ok(Value::Number(a / b))
                    }
                    (Value::Number(a), Value::Number(b), ArithmeticOperator::Multiply) => {
                        Ok(Value::Number(a * b))
                    }
                    (Value::Number(a), Value::Number(b), ArithmeticOperator::Subtract) => {
                        Ok(Value::Number(a - b))
                    }
                    _ => Err(InterpreterError::InvalidAddition(a, b)),
                }
            }
            Operation::Comparison {
                operator: _,
                a: _,
                b: _,
            } => todo!(),
            Operation::Group(_group) => todo!(),
            Operation::Logical {
                operator: _,
                expression: _,
            } => todo!(),
        }
    }

    fn visit_primitive(
        &self,
        primitive: &super::tree::primitive::Primitive,
    ) -> Result<Value, InterpreterError> {
        Ok(match primitive {
            Primitive::Boolean(value) => Value::Boolean(*value),
            Primitive::Nil => Value::Nil,
            Primitive::Number(value) => Value::Number(*value),
            Primitive::String(string) => Value::String(string.clone()),
        })
    }
}
