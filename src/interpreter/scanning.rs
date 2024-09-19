use core::fmt;

pub struct Scanner<'a> {
    input: &'a str,
    index: usize,
    fused: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            fused: false,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() || self.input[self.index..].is_empty() {
            if self.fused {
                return None;
            } else {
                self.fused = true;
                return Some(Ok(Token::Eof));
            }
        }

        let mut rest = self.input[self.index..].chars();
        let token = match rest.next()? {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            other => return Some(Err(ScannerError::UnknownCharacter(other))),
        };

        self.index += 1;

        Some(Ok(token))
    }
}

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char),
}

#[derive(Debug)]
pub enum Token {
    Eof,
    LeftBrace,
    LeftParenthesis,
    RightBrace,
    RightParenthesis,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Eof => "EOF  null",
                Self::LeftBrace => "LEFT_BRACE { null",
                Self::LeftParenthesis => "LEFT_PAREN ( null",
                Self::RightBrace => "RIGHT_BRACE } null",
                Self::RightParenthesis => "RIGHT_PAREN ) null",
            }
        )
    }
}
