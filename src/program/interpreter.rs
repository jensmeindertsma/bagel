use super::tree::{
    expression::{
        operator::{ArithmeticOperator, ComparisonOperator, LogicalOperator},
        Expression, ExpressionKind, Operation, Primitive,
    },
    statement::Statement,
    visitor::Visitor,
    Tree, TreeKind,
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

    pub fn run(&mut self) -> Result<(), InterpreterError> {}

    fn visit_tree(&self, tree: &Tree) -> Result<Value, InterpreterError> {
        match &tree.kind {
            TreeKind::Expression(operation) => self.visit_expression(operation),
            TreeKind::Statement(primitive) => self.visit_statement(primitive),
        }
    }
}

impl Visitor<Result<Value, InterpreterError>> for Interpreter {
    fn visit_expression(&self, expression: &Expression) -> Result<Value, InterpreterError> {
        match &expression.kind {
            ExpressionKind::Operation(operation) => match operation {
                Operation::Arithmetic { operator, a, b } => {
                    let line = a.line;

                    let a = self.visit_expression(a)?;
                    let b = self.visit_expression(b)?;

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

                    let a = self.visit_expression(a)?;
                    let b = self.visit_expression(b)?;

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
                        (_operator, _a, _b) => Err(InterpreterError::new(
                            // ErrorKind::Comparison {
                            //     operator: *operator,
                            //     a,
                            //     b,
                            // },
                            ErrorKind::Comparison,
                            line,
                        )),
                    }
                }

                Operation::Group(group) => self.visit_expression(group),

                Operation::Logical {
                    operator,
                    expression,
                } => {
                    let line = expression.line;

                    let value = self.visit_expression(expression)?;

                    match (operator, &value) {
                        (LogicalOperator::Negate, Value::Number(number)) => {
                            Ok(Value::Number(-number))
                        }
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
            },

            ExpressionKind::Primitive(primitive) => Ok(match primitive {
                Primitive::Boolean(value) => Value::Boolean(*value),
                Primitive::Nil => Value::Nil,
                Primitive::Number(value) => Value::Number(*value),
                Primitive::String(string) => Value::String(string.clone()),
            }),
        }
    }

    fn visit_statement(&self, statement: &Statement) -> Result<Value, InterpreterError> {
        todo!()
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
    // Comparison {
    //     operator: ComparisonOperator,
    //     a: Value,
    //     b: Value,
    // },
    Comparison,
    Division(Value, Value),
    Multiplication(Value, Value),
    Subtraction(Value, Value),
    Negation(Value),
}

// Obviously these error messages are not ideal as they do not show
// the operands but this format is demanded by CodeCrafters.
impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Addition(_a, _b) => {
                write!(
                    f,
                    "Operands must be two numbers or two strings.\n[line {}]",
                    self.line
                )
            }
            // Here I would extract `operator`, `a` and `b` and print different
            // error messages based on the operator.
            ErrorKind::Comparison => write!(f, "Operands must be numbers.\n[line {}]", self.line),
            ErrorKind::Division(_a, _b)
            | ErrorKind::Multiplication(_a, _b)
            | ErrorKind::Subtraction(_a, _b) => {
                write!(f, "Operands must be numbers.\n[line {}]", self.line)
            }

            ErrorKind::Negation(_value) => {
                write!(f, "Operand must be a number.\n[line {}]", self.line)
            }
        }
    }
}

impl Error for InterpreterError {}
