use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    process::ExitCode,
};

pub fn run(arguments: impl IntoIterator<Item = String>) -> Result<(), InterpreterError> {
    for argument in arguments {
        println!("argument = {argument}")
    }

    // TODO: match command and read input file.
    // if no command start accepting playground
    // here we construct our own pretty printer

    Ok(())
}

#[derive(Clone, Debug)]
pub enum InterpreterError {
    Scanner(Vec<usize>),
}

impl InterpreterError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Scanner(_) => ExitCode::from(65),
        }
    }
}
