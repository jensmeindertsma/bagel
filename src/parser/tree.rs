use core::fmt::{self, Formatter};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Tree {
    Operation {
        operator: Operator,
        arguments: Vec<Tree>,
    },
    Primitive(Primitive),
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation {
                operator,
                arguments,
            } => {
                write!(f, "({operator}")?;

                for argument in arguments {
                    write!(f, " {argument}")?;
                }

                write!(f, ")")
            }
            Self::Primitive(primitive) => write!(f, "{primitive}"),
        }
    }
}

#[derive(Debug)]
pub enum Primitive {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => {
                if *n == n.trunc() {
                    // Number is integer, but those should be formatted with a trailing zero.
                    write!(f, "{n}.0")
                } else {
                    write!(f, "{n}")
                }
            }
            Self::String(string) => write!(f, "{string}"),
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Division,
    Group,
    Multiplication,
    Negation,
    Not,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Division => write!(f, "/"),
            Self::Group => write!(f, "group"),
            Self::Multiplication => write!(f, "*"),
            Self::Negation => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Operator {
    pub fn binding_power(&self) -> (Option<u8>, Option<u8>) {
        match self {
            Self::Division | Self::Multiplication => (Some(32), Some(33)),
            Self::Group => (None, None),
            Self::Negation | Self::Not => (None, Some(u8::MAX)),
        }
    }
}
