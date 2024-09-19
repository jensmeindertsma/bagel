use core::fmt;
use std::str::Chars;

pub struct Scanner<'a> {
    characters: Chars<'a>,
    done: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars(),
            done: false,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.characters.next() {
            None => {
                if self.done {
                    None
                } else {
                    self.done = true;
                    Some(Ok(Token::Eof))
                }
            }
            Some(character) => match character {
                ',' => Some(Ok(Token::Comma)),
                '.' => Some(Ok(Token::Dot)),
                '{' => Some(Ok(Token::LeftBrace)),
                '(' => Some(Ok(Token::LeftParenthesis)),
                '-' => Some(Ok(Token::Minus)),
                '+' => Some(Ok(Token::Plus)),
                '}' => Some(Ok(Token::RightBrace)),
                ')' => Some(Ok(Token::RightParenthesis)),
                ';' => Some(Ok(Token::Semicolon)),
                '*' => Some(Ok(Token::Star)),
                other => Some(Err(ScannerError::UnknownCharacter {
                    character: other,
                    line: 1,
                })),
            },
        }
    }
}

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter { character: char, line: usize },
}

#[derive(Debug)]
pub enum Token {
    Comma,
    Dot,
    Eof,
    LeftBrace,
    LeftParenthesis,
    Minus,
    Plus,
    RightBrace,
    RightParenthesis,
    Semicolon,
    Star,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Comma => "COMMA , null",
                Self::Dot => "DOT . null",
                Self::Eof => "EOF  null",
                Self::LeftBrace => "LEFT_BRACE { null",
                Self::LeftParenthesis => "LEFT_PAREN ( null",
                Self::Minus => "MINUS - null",
                Self::Plus => "PLUS + null",
                Self::RightBrace => "RIGHT_BRACE } null",
                Self::RightParenthesis => "RIGHT_PAREN ) null",
                Self::Semicolon => "SEMICOLON ; null",
                Self::Star => "STAR * null",
            }
        )
    }
}
