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
        tracing::info!("producing token `{kind:?}` (line {})", self.current_line);

        Some(Ok(Token {
            kind,
            line: self.current_line,
        }))
    }

    fn fail(&self, error: ScannerError) -> Option<Result<Token, ScannerError>> {
        tracing::error!("producing error `{error:?}`");
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

            if !matches!(next_character, ' ' | '\t' | '\n') {
                tracing::debug!("scanning next token `{}`", next_character);
            }

            match next_character {
                ' ' | '\t' => continue,

                '\n' => {
                    self.current_line += 1;
                    tracing::debug!("now scanning line {}", self.current_line);
                    continue;
                }

                '(' => return self.produce(TokenKind::LeftParenthesis),
                ')' => return self.produce(TokenKind::RightParenthesis),

                '{' => return self.produce(TokenKind::LeftBrace),
                '}' => return self.produce(TokenKind::RightBrace),

                ',' => return self.produce(TokenKind::Comma),
                '.' => return self.produce(TokenKind::Dot),
                ';' => return self.produce(TokenKind::Semicolon),

                '!' => return self.if_next_else('=', TokenKind::BangEqual, TokenKind::Bang),
                '=' => return self.if_next_else('=', TokenKind::EqualEqual, TokenKind::Equal),

                '<' => return self.if_next_else('=', TokenKind::LessEqual, TokenKind::Less),
                '>' => return self.if_next_else('=', TokenKind::GreaterEqual, TokenKind::Greater),

                '+' => return self.produce(TokenKind::Plus),
                '-' => return self.produce(TokenKind::Minus),
                '*' => return self.produce(TokenKind::Star),
                '/' => {
                    // Here we need to handle both slash tokens and comments.
                    //
                    // 1. We peek at the next character and yield a slash token
                    // if it's not a slash.
                    //
                    // 2. If it is a slash we enter a loop of peek & consume:
                    // we consume (ignore) the next characters until we hit the
                    // end of the line. We do not support multi line comments.

                    if let Some('/') = self.characters.peek() {
                        while let Some(c) = self.characters.peek() {
                            if *c == '\n' {
                                break;
                            } else {
                                self.characters.next();
                            }
                        }

                        // End of line, resume the scanning loop.
                        self.current_line += 1;

                        continue;
                    } else {
                        return self.produce(TokenKind::Slash);
                    }
                }

                '"' => {
                    // Handling strings is much like handling comments
                    // above, except we keep track of the characters
                    // we encounter. We also support multi-line strings
                    // so we need to handle newline characters.

                    tracing::trace!("handling string");

                    let mut string = String::new();
                    while let Some(c) = self.characters.peek() {
                        match *c {
                            '"' => {
                                tracing::trace!("reached end of string `{string}`");
                                self.characters.next();
                                return self.produce(TokenKind::String(string));
                            }
                            '\n' => {
                                self.characters.next();
                                tracing::trace!("encountered newline while handling string");
                            }
                            character => {
                                self.characters.next();
                                tracing::trace!("adding new character `{character}`to the string");
                                string.push(character)
                            }
                        }
                    }

                    tracing::warn!("reached EOF without closing string");
                    return self.fail(ScannerError::UnexpectedEndOfFile {
                        line: self.current_line,
                    });
                }

                other => {
                    return self.fail(ScannerError::UnexpectedCharacter {
                        character: other,
                        line: self.current_line,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    String(String),
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
        // For STRING/NUMBER tokens, it is the value of the string/number.

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
            Self::String(string) => write!(formatter, "STRING \"{string}\" {string}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    UnexpectedCharacter { character: char, line: usize },
    UnexpectedEndOfFile { line: usize },
}

impl fmt::Display for ScannerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter { character, line } => write!(
                formatter,
                "unexpected character `{character}` on line {line}"
            ),
            Self::UnexpectedEndOfFile { line } => {
                write!(formatter, "unexpected end of line on line {line}")
            }
        }
    }
}

impl Error for ScannerError {}
