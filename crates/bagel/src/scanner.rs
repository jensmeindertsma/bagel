use core::fmt::{self, Formatter};
use std::{error::Error, iter::Peekable, str::Chars};

use tracing::instrument;

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

    fn if_next_else(
        &mut self,
        expected: char,
        a: TokenKind,
        b: TokenKind,
    ) -> Option<Result<Token, ScannerError>> {
        if let Some(found) = self.characters.peek()
            && *found == expected
        {
            self.characters.next();
            return self.produce(a);
        } else {
            return self.produce(b);
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        tracing::info!("next token");
        loop {
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

            return match next_character {
                ' ' | '\t' => continue,

                '\n' => {
                    self.current_line += 1;
                    continue;
                }

                '!' => self.if_next_else('=', TokenKind::BangEqual, TokenKind::Bang),

                ',' => self.produce(TokenKind::Comma),
                '.' => self.produce(TokenKind::Dot),

                '=' => self.if_next_else('=', TokenKind::EqualEqual, TokenKind::Equal),

                '>' => self.if_next_else('=', TokenKind::GreaterEqual, TokenKind::Greater),

                '{' => self.produce(TokenKind::LeftBrace),
                '(' => self.produce(TokenKind::LeftParenthesis),

                '<' => self.if_next_else('=', TokenKind::LessEqual, TokenKind::Less),

                '-' => self.produce(TokenKind::Minus),
                '+' => self.produce(TokenKind::Plus),
                '}' => self.produce(TokenKind::RightBrace),
                ')' => self.produce(TokenKind::RightParenthesis),
                ';' => self.produce(TokenKind::Semicolon),

                '/' => {
                    tracing::trace!("found /");
                    if let Some('/') = self.characters.peek() {
                        tracing::trace!("found second / -> //");
                        // Handling comments by going through every
                        // character until we hit a newline.
                        while let Some(c) = self.characters.peek() {
                            if *c == '\n' {
                                // Don't forget to increase line counter.
                                self.current_line += 1;
                                break;
                            }
                            // Not a newline, move to next character.
                            self.characters.next();
                        }

                        // Move on back into the scanning loop.
                        continue;
                    } else {
                        return self.produce(TokenKind::Slash);
                    }
                }

                '*' => self.produce(TokenKind::Star),

                other => {
                    return self.fail(ScannerError::UnexpectedCharacter {
                        character: other,
                        line: self.current_line,
                    });
                }
            };
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
    Bang,
    BangEqual,
    Comma,
    Dot,
    EndOfFile,
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
    Slash,
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
            Self::Bang => write!(formatter, "BANG ! null"),
            Self::BangEqual => write!(formatter, "BANG_EQUAL != null"),
            Self::Comma => write!(formatter, "COMMA , null"),
            Self::Dot => write!(formatter, "DOT . null"),
            Self::EndOfFile => write!(formatter, "EOF  null"),
            Self::Equal => write!(formatter, "EQUAL = null"),
            Self::EqualEqual => write!(formatter, "EQUAL_EQUAL == null"),
            Self::Greater => write!(formatter, "GREATER > null"),
            Self::GreaterEqual => write!(formatter, "GREATER_EQUAL >= null"),
            Self::LeftBrace => write!(formatter, "LEFT_BRACE {{ null"),
            Self::LeftParenthesis => write!(formatter, "LEFT_PAREN ( null"),
            Self::Less => write!(formatter, "LESS < null"),
            Self::LessEqual => write!(formatter, "LESS_EQUAL <= null"),
            Self::Minus => write!(formatter, "MINUS - null"),
            Self::Plus => write!(formatter, "PLUS + null"),
            Self::RightBrace => write!(formatter, "RIGHT_BRACE }} null"),
            Self::RightParenthesis => write!(formatter, "RIGHT_PAREN ) null"),
            Self::Semicolon => write!(formatter, "SEMICOLON ; null"),
            Self::Slash => write!(formatter, "SLASH / null"),
            Self::Star => write!(formatter, "STAR * null"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    UnexpectedCharacter { character: char, line: usize },
}

impl fmt::Display for ScannerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter { character, line } => write!(
                formatter,
                "unexpected character `{character}` on line {line}"
            ),
        }
    }
}

impl Error for ScannerError {}
