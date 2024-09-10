mod scanning;

pub use scanning::ScannerError;
use scanning::{Scanner, Token};

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, input: &str) -> Result<String, InterpreterError> {
        let tokens = self.scan(input)?;

        // TODO: implement this method
        Ok(format!("{tokens:?}"))
    }

    pub fn scan(&self, input: &str) -> Result<Vec<Token>, InterpreterError> {
        let mut tokens = Vec::new();

        for result in Scanner::new(input) {
            match result {
                Ok(token) => tokens.push(token),
                Err(scanner_error) => return Err(InterpreterError::Scanner(scanner_error)),
            }
        }

        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    Scanner(ScannerError),
}
