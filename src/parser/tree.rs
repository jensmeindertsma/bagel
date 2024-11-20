use core::fmt::{self, Formatter};

#[derive(Debug)]
pub enum Tree {
    Primitive(Primitive),
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(primitive) => primitive.fmt(f),
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
