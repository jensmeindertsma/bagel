use core::fmt;
use std::{iter::Peekable, str::Chars};

pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
    done: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars().peekable(),
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
            Some(character) => {
                enum Started {
                    IfNextEqual { then: Token, otherwise: Token },
                }

                let started = match character {
                    ',' => return Some(Ok(Token::Comma)),
                    '.' => return Some(Ok(Token::Dot)),
                    '{' => return Some(Ok(Token::LeftBrace)),
                    '(' => return Some(Ok(Token::LeftParenthesis)),
                    '-' => return Some(Ok(Token::Minus)),
                    '+' => return Some(Ok(Token::Plus)),
                    '}' => return Some(Ok(Token::RightBrace)),
                    ')' => return Some(Ok(Token::RightParenthesis)),
                    ';' => return Some(Ok(Token::Semicolon)),
                    '*' => return Some(Ok(Token::Star)),

                    '!' => Started::IfNextEqual {
                        then: Token::BangEqual,
                        otherwise: Token::Bang,
                    },

                    '=' => Started::IfNextEqual {
                        then: Token::EqualEqual,
                        otherwise: Token::Equal,
                    },

                    '<' => Started::IfNextEqual {
                        then: Token::LessEqual,
                        otherwise: Token::Less,
                    },

                    '>' => Started::IfNextEqual {
                        then: Token::GreaterEqual,
                        otherwise: Token::Greater,
                    },

                    other => {
                        return Some(Err(ScannerError::UnknownCharacter {
                            character: other,
                            line: 1,
                        }))
                    }
                };

                let full_token = match started {
                    Started::IfNextEqual { then, otherwise } => {
                        let next = self.characters.peek();

                        if next == Some(&'=') {
                            self.characters.next();
                            then
                        } else {
                            otherwise
                        }
                    }
                };

                Some(Ok(full_token))
            }
        }
    }
}

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter { character: char, line: usize },
}

#[derive(Debug)]
pub enum Token {
    Bang,
    BangEqual,
    Comma,
    Dot,
    Eof,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    LeftBrace,
    LeftParenthesis,
    Less,
    LessEqual,
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
                Self::Bang => "BANG ! null",
                Self::BangEqual => "BANG_EQUAL != null",
                Self::Comma => "COMMA , null",
                Self::Dot => "DOT . null",
                Self::Eof => "EOF  null",
                Self::Equal => "EQUAL = null",
                Self::EqualEqual => "EQUAL_EQUAL == null",
                Self::Greater => "GREATER > null",
                Self::GreaterEqual => "GREATER_EQUAL >= null",
                Self::LeftBrace => "LEFT_BRACE { null",
                Self::LeftParenthesis => "LEFT_PAREN ( null",
                Self::Less => "LESS < null",
                Self::LessEqual => "LESS_EQUAL <= null",
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
