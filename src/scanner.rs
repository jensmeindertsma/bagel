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

                        // Check whether identifier is actually
                        // reserved keyword
                        match lexeme.as_str() {
                            "and" => Token::And,
                            "class" => Token::Class,
                            "else" => Token::Else,
                            "false" => Token::False,
                            "for" => Token::For,
                            "fun" => Token::Fun,
                            "if" => Token::If,
                            "nil" => Token::Nil,
                            "or" => Token::Or,
                            "print" => Token::Print,
                            "return" => Token::Return,
                            "super" => Token::Super,
                            "this" => Token::This,
                            "true" => Token::True,
                            "var" => Token::Var,
                            "while" => Token::While,
                            _ => Token::Identifier { lexeme },
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

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCharacter { character, line } => {
                write!(f, "unexpected character `{character}` on line {line}")
            }
            Self::UnterminatedString { line } => {
                write!(f, "unterminated string on line {line}")
            }
        }
    }
}

impl Error for ScannerError {}

#[derive(Clone, Debug)]
pub enum Token {
    And,
    Bang,
    BangEqual,
    Class,
    Comma,
    Dot,
    Else,
    Eof,
    Equal,
    EqualEqual,
    False,
    For,
    Fun,
    Greater,
    GreaterEqual,
    Identifier { lexeme: String },
    If,
    LeftBrace,
    LeftParenthesis,
    Less,
    LessEqual,
    Minus,
    Nil,
    Number { literal: String, value: f64 },
    Or,
    Plus,
    Print,
    Return,
    RightBrace,
    RightParenthesis,
    Semicolon,
    Slash,
    Star,
    String { value: String },
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => write!(f, "AND and null"),
            Self::Bang => write!(f, "BANG ! null"),
            Self::BangEqual => write!(f, "BANG_EQUAL != null"),
            Self::Class => write!(f, "CLASS class null"),
            Self::Comma => write!(f, "COMMA , null"),
            Self::Dot => write!(f, "DOT . null"),
            Self::Eof => write!(f, "EOF  null"),
            Self::Else => write!(f, "ELSE else null"),
            Self::Equal => write!(f, "EQUAL = null"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Self::False => write!(f, "FALSE false null"),
            Self::For => write!(f, "FOR for null"),
            Self::Fun => write!(f, "FUN fun null"),
            Self::Greater => write!(f, "GREATER > null"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Self::Identifier { lexeme } => write!(f, "IDENTIFIER {lexeme} null"),
            Self::If => write!(f, "IF if null"),
            Self::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Self::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Self::Less => write!(f, "LESS < null"),
            Self::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Self::Minus => write!(f, "MINUS - null"),
            Self::Nil => write!(f, "NIL nil null"),
            Self::Number { literal, value } => {
                if *value == value.trunc() {
                    write!(f, "NUMBER {literal} {value}.0")
                } else {
                    write!(f, "NUMBER {literal} {value}")
                }
            }
            Self::Or => write!(f, "OR or null"),
            Self::Plus => write!(f, "PLUS + null"),
            Self::Print => write!(f, "PRINT print null"),
            Self::Return => write!(f, "RETURN return null"),
            Self::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Self::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),
            Self::Semicolon => write!(f, "SEMICOLON ; null"),
            Self::Slash => write!(f, "SLASH / null"),
            Self::Star => write!(f, "STAR * null"),
            Self::String { value } => write!(f, "STRING \"{value}\" {value}"),
            Self::Super => write!(f, "SUPER super null"),
            Self::This => write!(f, "THIS this null"),
            Self::True => write!(f, "TRUE true null"),
            Self::Var => write!(f, "VAR var null"),
            Self::While => write!(f, "WHILE while null"),
        }
    }
}
