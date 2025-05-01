use super::tree::{
    operation::Operation,
    operator::{ArithmeticOperator, LogicalOperator},
    primitive::Primitive,
    visitor::Visitor,
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
    Addition(Value, Value),
    Division(Value, Value),
    Multiplication(Value, Value),
    Subtraction(Value, Value),
    Negation(Value),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Addition(a, b) => {
                write!(f, "invalid addition `{a}` + `{b}`")
            }
            Self::Division(a, b) => {
                write!(f, "invalid division `{a}` / `{b}`")
            }
            Self::Multiplication(a, b) => {
                write!(f, "invalid multiplication `{a}` * `{b}`")
            }
            Self::Subtraction(a, b) => {
                write!(f, "invalid subtraction `{a}` - `{b}`")
            }
            Self::Negation(value) => {
                write!(f, "invalid negation of `{value}`")
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

                match (operator, &a, &b) {
                    (ArithmeticOperator::Add, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a + b))
                    }

                    (ArithmeticOperator::Divide, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a / b))
                    }
                    (ArithmeticOperator::Multiply, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a * b))
                    }
                    (ArithmeticOperator::Subtract, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a - b))
                    }

                    _ => match operator {
                        ArithmeticOperator::Add => Err(InterpreterError::Addition(a, b)),
                        ArithmeticOperator::Divide => Err(InterpreterError::Division(a, b)),
                        ArithmeticOperator::Multiply => Err(InterpreterError::Multiplication(a, b)),
                        ArithmeticOperator::Subtract => Err(InterpreterError::Subtraction(a, b)),
                    },
                }
            }
            Operation::Comparison {
                operator: _,
                a: _,
                b: _,
            } => todo!(),
            Operation::Group(group) => self.visit_tree(group),
            Operation::Logical {
                operator,
                expression,
            } => {
                let value = self.visit_tree(expression)?;

                match (operator, &value) {
                    (LogicalOperator::Negate, Value::Number(number)) => Ok(Value::Number(-number)),
                    (LogicalOperator::Negate, _) => Err(InterpreterError::Negation(value)),
                    (LogicalOperator::Not, value) => Ok(Value::Boolean(match value {
                        Value::Boolean(value) => !value,
                        Value::Nil => true,
                        Value::Number(_) => false,
                        Value::String(_) => false,
                    })),
                }
            }
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
