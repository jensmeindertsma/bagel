use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
    current_line: usize,
    offset: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars().peekable(),
            current_line: 1,
            offset: 0,
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = match self.characters.next().ok_or(ScannerError::UnexpectedEnd)? {
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
        };

        self.offset += 1;

        Some(Ok(Token { kind }))
    }
}

pub struct Token {
    kind: TokenKind,
}

impl Display for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "todo")
    }
}

pub enum TokenKind {
    LeftParenthesis,
    RightParenthesis,
}

impl Display for TokenKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParenthesis => write!(formatter, "LEFT_PAREN ( null"),
            Self::RightParenthesis => write!(formatter, "RIGHT_PAREN ) null"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScannerError {
    line: usize,
    start: usize,
    end: usize,
}

pub enum ErrorKind {
    UnexpectedCharacter { line: usize, character: char },
    UnexpectedEnd { line: usize },
}

impl Display for ScannerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter { line, character } => {
                write!(
                    formatter,
                    "[line {line}] Error: Unexpected character: {character}"
                )
            }
            Self::UnexpectedEnd { line } => {
                write!(formatter, "[line {line}: Unexpected end of input")
            }
        }
    }
}

impl Error for ScannerError {}
