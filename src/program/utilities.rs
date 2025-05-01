use super::{
    scanning::{Scanner, ScannerError},
    token::Token,
};

pub fn tokenize(input: &str) -> Result<Vec<Token>, Vec<ScannerError>> {
    let scanner = Scanner::new(input);

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for output in scanner {
        match output {
            Ok(token) => tokens.push(token),
            Err(error) => errors.push(error),
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(tokens)
    }
}
