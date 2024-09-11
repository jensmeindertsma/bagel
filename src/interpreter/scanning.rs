use core::fmt;

pub struct Scanner<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, index: 0 }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            if self.index == 0 {
                self.index += 1;

                return Some(Ok(Token::Eof));
            } else {
                return None;
            }
        }

        Some(Err(ScannerError::Unknown))
    }
}

#[derive(Debug)]
pub enum ScannerError {
    Unknown,
}

#[derive(Debug)]
pub enum Token {
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Eof => "EOF  null",
            }
        )
    }
}
