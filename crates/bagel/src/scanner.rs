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

    fn fail(
        &self,
        closure: impl FnOnce(usize) -> ScannerError,
    ) -> Option<Result<Token, ScannerError>> {
        let error = closure(self.current_line);

        Some(Err(error))
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
                    self.produce(TokenKind::EndOfFile)
                }
            }
            Some(character) => {
                tracing::trace!("processing next character `{character}`");

                let token_kind = match character {
                    '\n' => {
                        self.current_line += 1;
                        return self.next();
                    }
                    '{' => TokenKind::LeftBrace,
                    '}' => TokenKind::RightBrace,
                    '(' => TokenKind::LeftParenthesis,
                    ')' => TokenKind::RightParenthesis,

                    _ => {
                        return self
                            .fail(|line| ScannerError::UnexpectedCharacter { character, line });
                    }
                };

                tracing::trace!("producing new token `{token_kind:?}`");

                self.produce(token_kind)
            }
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
    EndOfFile,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
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
            Self::EndOfFile => write!(formatter, "EOF  null"),
            Self::LeftBrace => write!(formatter, "LEFT_BRACE {{ null"),
            Self::RightBrace => write!(formatter, "RIGHT_BRACE }} null"),
            Self::LeftParenthesis => write!(formatter, "LEFT_PAREN ( null"),
            Self::RightParenthesis => write!(formatter, "RIGHT_PAREN ) null"),
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
