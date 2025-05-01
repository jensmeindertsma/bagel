use super::tree::{
    operation::Operation,
    operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator},
    primitive::Primitive,
    visitor::Visitor,
    Kind, Tree,
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
        match &tree.kind {
            Kind::Operation(operation) => self.visit_operation(operation),
            Kind::Primitive(primitive) => self.visit_primitive(primitive),
        }
    }
}

#[derive(Debug, PartialEq)]
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
pub struct InterpreterError {
    kind: ErrorKind,
    line: usize,
}

impl InterpreterError {
    fn new(kind: ErrorKind, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Addition(Value, Value),
    Comparison {
        operator: ComparisonOperator,
        a: Value,
        b: Value,
    },
    Division(Value, Value),
    Multiplication(Value, Value),
    Subtraction(Value, Value),
    Negation(Value),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Addition(a, b) => {
                write!(f, "invalid addition `{a}` + `{b}`")
            }
            ErrorKind::Comparison { operator, a, b } => match operator {
                ComparisonOperator::Equal => write!(f, "invalid comparison `{a}` == `{b}`"),
                ComparisonOperator::GreaterEqual => write!(f, "invalid comparison `{a}` >= `{b}`"),
                ComparisonOperator::GreaterThan => write!(f, "invalid comparison `{a}` > `{b}`"),
                ComparisonOperator::LessEqual => write!(f, "invalid comparison `{a}` <= `{b}`"),
                ComparisonOperator::LessThan => write!(f, "invalid comparison `{a}` < `{b}`"),
                ComparisonOperator::NotEqual => write!(f, "invalid comparison `{a}` != `{b}`"),
            },
            ErrorKind::Division(a, b) => {
                write!(f, "invalid division `{a}` / `{b}`")
            }
            ErrorKind::Multiplication(a, b) => {
                write!(f, "invalid multiplication `{a}` * `{b}`")
            }
            ErrorKind::Subtraction(a, b) => {
                write!(f, "invalid subtraction `{a}` - `{b}`")
            }
            ErrorKind::Negation(_value) => {
                write!(f, "Operand must be a number.\n[line {}]", self.line)
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
                let line = a.line;

                let a = self.visit_tree(a)?;
                let b = self.visit_tree(b)?;

                match (operator, &a, &b) {
                    // String concatenation overloads the `+` operator so it
                    // must come before the numeric addition operator using the
                    // same keyword.
                    (ArithmeticOperator::Add, Value::String(a), Value::String(b)) => {
                        let mut new = a.clone();
                        new.push_str(b);
                        Ok(Value::String(new))
                    }

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
                        ArithmeticOperator::Add => {
                            Err(InterpreterError::new(ErrorKind::Addition(a, b), line))
                        }
                        ArithmeticOperator::Divide => {
                            Err(InterpreterError::new(ErrorKind::Division(a, b), line))
                        }
                        ArithmeticOperator::Multiply => {
                            Err(InterpreterError::new(ErrorKind::Multiplication(a, b), line))
                        }
                        ArithmeticOperator::Subtract => {
                            Err(InterpreterError::new(ErrorKind::Subtraction(a, b), line))
                        }
                    },
                }
            }

            Operation::Comparison { operator, a, b } => {
                let line = a.line;

                let a = self.visit_tree(a)?;
                let b = self.visit_tree(b)?;

                match (operator, a, b) {
                    (ComparisonOperator::Equal, a, b) => Ok(Value::Boolean(a == b)),
                    (ComparisonOperator::GreaterEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a >= b))
                    }
                    (ComparisonOperator::GreaterThan, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a > b))
                    }
                    (ComparisonOperator::LessEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a <= b))
                    }
                    (ComparisonOperator::LessThan, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Boolean(a < b))
                    }
                    (ComparisonOperator::NotEqual, a, b) => Ok(Value::Boolean(a != b)),
                    (operator, a, b) => Err(InterpreterError::new(
                        ErrorKind::Comparison {
                            operator: *operator,
                            a,
                            b,
                        },
                        line,
                    )),
                }
            }

            Operation::Group(group) => self.visit_tree(group),

            Operation::Logical {
                operator,
                expression,
            } => {
                let line = expression.line;

                let value = self.visit_tree(expression)?;

                match (operator, &value) {
                    (LogicalOperator::Negate, Value::Number(number)) => Ok(Value::Number(-number)),
                    (LogicalOperator::Negate, _) => {
                        Err(InterpreterError::new(ErrorKind::Negation(value), line))
                    }
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
