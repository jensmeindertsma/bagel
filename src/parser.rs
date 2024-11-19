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
    T: Iterator<Item = Token> + Clone,
{
    pub fn new(tokens: T) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Tree, Vec<ParserError>> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, minimum_binding_power: u8) -> Result<Tree, Vec<ParserError>> {
        let left_hand_side = self.tokens.next().ok_or(ParserError::UnexpectedEOF);

        todo!()
    }
}

pub enum Tree {}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo")
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
