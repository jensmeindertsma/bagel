use core::fmt;
use std::{iter::Peekable, str::Chars};

pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
    done: bool,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars().peekable(),
            done: false,
            line: 1,
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
                    Identifier { first_char: char },
                    IfNextEqual { then: Token, otherwise: Token },
                    MaybeComment { otherwise: Token },
                    Number { first_digit: char },
                    String,
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

                    '/' => Started::MaybeComment {
                        otherwise: Token::Slash,
                    },

                    '"' => Started::String,

                    '\n' => {
                        self.line += 1;

                        return self.next();
                    }

                    c if c.is_whitespace() => return self.next(),

                    c if c.is_ascii_digit() => Started::Number { first_digit: c },

                    c if c.is_ascii_alphabetic() || c == '_' => {
                        Started::Identifier { first_char: c }
                    }

                    character => {
                        return Some(Err(ScannerError::UnknownCharacter {
                            character,
                            line: self.line,
                        }));
                    }
                };

                let full_token = match started {
                    Started::Identifier { first_char } => {
                        let mut lexeme = String::from(first_char);

                        loop {
                            match self.characters.peek() {
                                None => break,
                                Some(char) => {
                                    if char.is_ascii_alphanumeric() || *char == '_' {
                                        lexeme.push(*char);
                                        self.characters.next();
                                    } else {
                                        // Literal is over
                                        break;
                                    }
                                }
                            }
                        }

                        Token::Identifier { lexeme }
                    }
                    Started::IfNextEqual { then, otherwise } => {
                        if self.characters.peek().copied() == Some('=') {
                            self.characters.next();
                            then
                        } else {
                            otherwise
                        }
                    }
                    Started::MaybeComment { otherwise } => {
                        if self.characters.peek().copied() == Some('/') {
                            while let Some(char) = self.characters.peek() {
                                if *char == '\n' {
                                    break;
                                } else {
                                    self.characters.next();
                                };
                            }

                            return self.next();
                        } else {
                            otherwise
                        }
                    }
                    Started::String => {
                        let mut value = String::new();

                        loop {
                            match self.characters.peek() {
                                None => {
                                    return Some(Err(ScannerError::UnterminatedString {
                                        line: self.line,
                                    }))
                                }
                                Some(char) => {
                                    if *char == '"' {
                                        self.characters.next();
                                        break Token::String { value };
                                    } else {
                                        value.push(*char);
                                        self.characters.next();
                                    };
                                }
                            }
                        }
                    }
                    Started::Number { first_digit } => {
                        let mut literal = String::from(first_digit);

                        loop {
                            match self.characters.peek() {
                                None => {
                                    // The number is only a single character long

                                    break;
                                }
                                Some(char) => {
                                    if char.is_ascii_digit() || *char == '.' {
                                        literal.push(*char);
                                        self.characters.next();
                                    } else {
                                        // Encountered non-number characters, so the number is over.
                                        break;
                                    }
                                }
                            }
                        }

                        let value = literal.parse().unwrap();
                        Token::Number { literal, value }
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
    UnterminatedString { line: usize },
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
    Identifier { lexeme: String },
    LeftBrace,
    LeftParenthesis,
    Less,
    LessEqual,
    Minus,
    Number { literal: String, value: f64 },
    Plus,
    RightBrace,
    RightParenthesis,
    Semicolon,
    Slash,
    Star,
    String { value: String },
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bang => write!(f, "BANG ! null"),
            Self::BangEqual => write!(f, "BANG_EQUAL != null"),
            Self::Comma => write!(f, "COMMA , null"),
            Self::Dot => write!(f, "DOT . null"),
            Self::Eof => write!(f, "EOF  null"),
            Self::Equal => write!(f, "EQUAL = null"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Self::Greater => write!(f, "GREATER > null"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Self::Identifier { lexeme } => write!(f, "IDENTIFIER {lexeme} null"),
            Self::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Self::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Self::Less => write!(f, "LESS < null"),
            Self::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Self::Minus => write!(f, "MINUS - null"),
            Self::Number { literal, value } => {
                if *value == value.trunc() {
                    write!(f, "NUMBER {literal} {value}.0")
                } else {
                    write!(f, "NUMBER {literal} {value}")
                }
            }
            Self::Plus => write!(f, "PLUS + null"),
            Self::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Self::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),
            Self::Semicolon => write!(f, "SEMICOLON ; null"),
            Self::Slash => write!(f, "SLASH / null"),
            Self::Star => write!(f, "STAR * null"),
            Self::String { value } => write!(f, "STRING \"{value}\" {value}"),
        }
    }
}
