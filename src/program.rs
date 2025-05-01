mod command;

use crate::{
    parse::ParserError,
    scan::{Scanner, ScannerError, Token},
    Parser,
};
use command::{Command, CommandError};
use std::{fs, io};

pub fn run(arguments: impl Iterator<Item = String>) -> Result<(), Failure> {
    let command = Command::from_arguments(arguments).map_err(Failure::InvalidCommand)?;

    match command {
        Command::Help => {
            println!("Help is coming (soon)!")
        }

        Command::Tokenize { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let scanner = Scanner::new(&input);
            let mut errors = Vec::new();

            for output in scanner {
                match output {
                    Ok(token) => println!("{token}"),
                    Err(error) => errors.push(error),
                }
            }

            if !errors.is_empty() {
                return Err(Failure::Program(ProgramError::Tokenization(errors)));
            }
        }

        Command::Parse { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let tokens =
                tokenize(&input).map_err(|e| Failure::Program(ProgramError::Tokenization(e)))?;

            let mut parser = Parser::new(tokens);

            let tree = parser
                .parse()
                .map_err(|e| Failure::Program(ProgramError::Parsing(e)))?;

            println!("{tree}")
        }

        Command::Evaluate { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let tokens =
                tokenize(&input).map_err(|e| Failure::Program(ProgramError::Tokenization(e)))?;

            let mut parser = Parser::new(tokens);

            let tree = parser
                .parse()
                .map_err(|e| Failure::Program(ProgramError::Parsing(e)))?;

            println!("Evaluating tree: {tree:?}");

            todo!()
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Failure {
    InvalidCommand(CommandError),
    Program(ProgramError),
    Io(io::Error),
}

fn tokenize(input: &str) -> Result<Vec<Token>, Vec<ScannerError>> {
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

#[derive(Debug)]
pub enum ProgramError {
    Tokenization(Vec<ScannerError>),
    Parsing(ParserError),
}
