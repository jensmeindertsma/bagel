use core::fmt::{self, Formatter};
use std::error::Error;

use crate::scanner::Token;

pub struct Parser {
    input: Vec<Token>,
}

pub enum Tree {
    Foo,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self { input }
    }

    pub fn finish(self) -> Result<Tree, ParserError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo: implement this!")
    }
}

impl Error for ParserError {}
