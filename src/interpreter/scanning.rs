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
        todo!()
    }
}

#[derive(Debug)]
pub enum ScannerError {
    Unknown,
}

#[derive(Debug)]
pub struct Token {}
