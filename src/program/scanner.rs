use super::token::{Token, TokenKind};
use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable, str::Chars};

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

impl Iterator for Scanner<'_> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.characters.next() {
            None => {
                if self.done {
                    None
                } else {
                    self.done = true;
                    Some(Ok(Token::new(TokenKind::Eof, self.line)))
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
                    ',' => return Some(Ok(Token::new(TokenKind::Comma, self.line))),
                    '.' => return Some(Ok(Token::new(TokenKind::Dot, self.line))),
                    '{' => return Some(Ok(Token::new(TokenKind::LeftBrace, self.line))),
                    '(' => return Some(Ok(Token::new(TokenKind::LeftParenthesis, self.line))),
                    '-' => return Some(Ok(Token::new(TokenKind::Minus, self.line))),
                    '+' => return Some(Ok(Token::new(TokenKind::Plus, self.line))),
                    '}' => return Some(Ok(Token::new(TokenKind::RightBrace, self.line))),
                    ')' => return Some(Ok(Token::new(TokenKind::RightParenthesis, self.line))),
                    ';' => return Some(Ok(Token::new(TokenKind::Semicolon, self.line))),
                    '*' => return Some(Ok(Token::new(TokenKind::Star, self.line))),

                    '!' => Started::IfNextEqual {
                        then: Token::new(TokenKind::BangEqual, self.line),
                        otherwise: Token::new(TokenKind::Bang, self.line),
                    },

                    '=' => Started::IfNextEqual {
                        then: Token::new(TokenKind::EqualEqual, self.line),
                        otherwise: Token::new(TokenKind::Equal, self.line),
                    },

                    '<' => Started::IfNextEqual {
                        then: Token::new(TokenKind::LessEqual, self.line),
                        otherwise: Token::new(TokenKind::Less, self.line),
                    },

                    '>' => Started::IfNextEqual {
                        then: Token::new(TokenKind::GreaterEqual, self.line),
                        otherwise: Token::new(TokenKind::Greater, self.line),
                    },

                    '/' => Started::MaybeComment {
                        otherwise: Token::new(TokenKind::Slash, self.line),
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

                        // Check whether identifier is actually
                        // reserved keyword
                        match lexeme.as_str() {
                            "and" => Token::new(TokenKind::And, self.line),
                            "class" => Token::new(TokenKind::Class, self.line),
                            "else" => Token::new(TokenKind::Else, self.line),
                            "false" => Token::new(TokenKind::False, self.line),
                            "for" => Token::new(TokenKind::For, self.line),
                            "fun" => Token::new(TokenKind::Fun, self.line),
                            "if" => Token::new(TokenKind::If, self.line),
                            "nil" => Token::new(TokenKind::Nil, self.line),
                            "or" => Token::new(TokenKind::Or, self.line),
                            "print" => Token::new(TokenKind::Print, self.line),
                            "return" => Token::new(TokenKind::Return, self.line),
                            "super" => Token::new(TokenKind::Super, self.line),
                            "this" => Token::new(TokenKind::This, self.line),
                            "true" => Token::new(TokenKind::True, self.line),
                            "var" => Token::new(TokenKind::Var, self.line),
                            "while" => Token::new(TokenKind::While, self.line),
                            _ => Token::new(TokenKind::Identifier { lexeme }, self.line),
                        }
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
                                    }));
                                }
                                Some(char) => {
                                    if *char == '"' {
                                        self.characters.next();
                                        break Token::new(TokenKind::String { value }, self.line);
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
                        Token::new(TokenKind::Number { literal, value }, self.line)
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

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCharacter { character, line } => {
                write!(f, "[line {line}] Error: Unexpected character: {character}")
            }
            Self::UnterminatedString { line } => {
                write!(f, "[line {line}] Error: Unterminated string.")
            }
        }
    }
}

impl Error for ScannerError {}
