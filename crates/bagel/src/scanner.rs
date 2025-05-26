use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    characters: Peekable<Chars<'a>>,
    current_line: usize,
    done: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            characters: input.chars().peekable(),
            current_line: 1,
            done: false,
        }
    }

    fn produce(&self, kind: TokenKind) -> Option<Result<Token, ScannerError>> {
        Some(Ok(Token {
            kind,
            line: self.current_line,
        }))
    }

    fn fail(&self, error: ScannerError) -> Option<Result<Token, ScannerError>> {
        Some(Err(error))
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        enum Started {
            Either(TokenKind, TokenKind),
            Identifier,
            Number,
            String,
        }

        let next_character = match self.characters.next() {
            Some(character) => character,
            None => {
                if self.done {
                    return None;
                } else {
                    self.done = true;
                    return self.produce(TokenKind::EndOfFile);
                }
            }
        };

        tracing::debug!("processing token {}", next_character);

        let started = match next_character {
            ' ' | '\t' => return self.next(),

            '\n' => {
                self.current_line += 1;
                return self.next();
            }

            ',' => return self.produce(TokenKind::Comma),
            '.' => return self.produce(TokenKind::Dot),
            '=' => Started::Either(TokenKind::Equal, TokenKind::EqualEqual),
            '{' => return self.produce(TokenKind::LeftBrace),
            '(' => return self.produce(TokenKind::LeftParenthesis),
            '-' => return self.produce(TokenKind::Minus),
            '+' => return self.produce(TokenKind::Plus),
            '}' => return self.produce(TokenKind::RightBrace),
            ')' => return self.produce(TokenKind::RightParenthesis),
            ';' => return self.produce(TokenKind::Semicolon),
            '*' => return self.produce(TokenKind::Star),

            character => {
                return self.fail(ScannerError::UnexpectedCharacter {
                    character,
                    line: self.current_line,
                });
            }
        };

        tracing::debug!("handling multi-character token");

        match started {
            Started::Either(TokenKind::Equal, TokenKind::EqualEqual) => {
                if let Some('=') = self.characters.peek() {
                    self.characters.next();
                    return self.produce(TokenKind::EqualEqual);
                } else {
                    return self.produce(TokenKind::Equal);
                }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        // Defer formatting to `TokenKind`
        write!(formatter, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Comma,
    Dot,
    EndOfFile,
    Equal,
    EqualEqual,
    LeftBrace,
    LeftParenthesis,
    Minus,
    Plus,
    RightBrace,
    RightParenthesis,
    Semicolon,
    Star,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        // We format the enum variants into a standard format defined in the book:
        // <token_type> <lexeme> <literal>
        //
        // <token_type>: The type of the token.
        // Examples: VAR, IDENTIFIER, STRING, EOF etc.
        //
        // <lexeme>: The actual sequence of characters that formed the token.
        // Examples: var, breakfast, "bagels" etc.
        // For an EOF token, the lexeme is an empty string.
        //
        // <literal>: The literal value of the token.
        // For most tokens this is null.
        // For STRING/NUMBER tokens, it holds the value of the string/number.

        match self {
            Self::Comma => write!(formatter, "COMMA , null"),
            Self::Dot => write!(formatter, "DOT . null"),
            Self::EndOfFile => write!(formatter, "EOF  null"),
            Self::Equal => write!(formatter, "EQUAL = null"),
            Self::EqualEqual => write!(formatter, "EQUAL_EQUAL == null"),
            Self::LeftBrace => write!(formatter, "LEFT_BRACE {{ null"),
            Self::LeftParenthesis => write!(formatter, "LEFT_PAREN ( null"),
            Self::Minus => write!(formatter, "MINUS - null"),
            Self::Plus => write!(formatter, "PLUS + null"),
            Self::RightBrace => write!(formatter, "RIGHT_BRACE }} null"),
            Self::RightParenthesis => write!(formatter, "RIGHT_PAREN ) null"),
            Self::Semicolon => write!(formatter, "SEMICOLON ; null"),
            Self::Star => write!(formatter, "STAR * null"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    UnexpectedEndOfFile { line: usize },
    UnexpectedCharacter { character: char, line: usize },
}

impl fmt::Display for ScannerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::UnexpectedEndOfFile { line } => {
                write!(formatter, "unexpected end of file on line {line}")
            }
            Self::UnexpectedCharacter { character, line } => write!(
                formatter,
                "unexpected character `{character}` on line {line}"
            ),
        }
    }
}

impl Error for ScannerError {}
