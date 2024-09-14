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

        // TODO: add some debug logging here to figure out why it doesn't work!

        let mut rest = self.input[self.index..].chars();
        let token = match rest.next()? {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
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
    LeftParenthesis,
    RightParenthesis,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Eof => "EOF  null",
                Self::LeftParenthesis => "LEFT_PAREN ( null",
                Self::RightParenthesis => "RIGHT_PAREN ) null",
            }
        )
    }
}
