mod command;
mod interpreting;
mod parsing;
mod printer;
mod scanning;
mod token;
mod tree;
mod utilities;

use command::{Command, CommandError};
use interpreting::Interpreter;
use parsing::{Parser, ParserError};
use printer::Printer;
use scanning::{Scanner, ScannerError};
use std::{fs, io};
use utilities::tokenize;

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

            Printer::new(&tree).print();
        }

        Command::Evaluate { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let tokens =
                tokenize(&input).map_err(|e| Failure::Program(ProgramError::Tokenization(e)))?;

            let mut parser = Parser::new(tokens);

            let tree = parser
                .parse()
                .map_err(|e| Failure::Program(ProgramError::Parsing(e)))?;

            let mut interpreter = Interpreter::new(tree);

            interpreter.evaluate();
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

#[derive(Debug)]
pub enum ProgramError {
    Tokenization(Vec<ScannerError>),
    Parsing(ParserError),
}
