use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
    current_line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars().peekable(),
            current_line: 1,
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "todo")
    }
}

impl Error for ScannerError {}
