use crate::scanner::Token;
use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable};

pub struct Parser<T>
where
    T: Iterator,
{
    tokens: Peekable<T>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(tokens: T) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Tree, Vec<ParserError>> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Tree, Vec<ParserError>> {
        self.parse_expression_within(0)
    }

    fn parse_expression_within(
        &mut self,
        minimum_binding_power: u8,
    ) -> Result<Tree, Vec<ParserError>> {
        let token = self.tokens.next().ok_or(vec![ParserError::UnexpectedEOF])?;

        let tree = match token {
            Token::True => Tree::Primitive(Primitive::Boolean(true)),
            Token::False => Tree::Primitive(Primitive::Boolean(false)),
            Token::Nil => Tree::Primitive(Primitive::Nil),
            Token::Number { value, .. } => Tree::Primitive(Primitive::Number(value)),
            _ => return Err(vec![ParserError::UnexpectedToken(token)]),
        };

        Ok(tree)
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnexpectedToken(Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEOF => write!(f, "unexpected EOF"),
            Self::UnexpectedToken(token) => {
                write!(f, "found unexpected token `{token:?}`")
            }
        }
    }
}

impl Error for ParserError {}

pub enum Tree {
    Primitive(Primitive),
}

pub enum Primitive {
    Boolean(bool),
    Nil,
    Number(f64),
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => {
                if *n == n.trunc() {
                    write!(f, "{n}.0")
                } else {
                    write!(f, "{n}")
                }
            }
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(primitive) => write!(f, "{primitive}"),
        }
    }
}
