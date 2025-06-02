use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars().peekable(),
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct Token {
    kind: TokenKind,
    start: usize,
    end: usize,
}

impl Display for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "todo")
    }
}

pub enum TokenKind {}

#[derive(Debug)]
pub enum ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "todo")
    }
}

impl Error for ScannerError {}
