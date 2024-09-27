mod command;
mod parser;
mod scanner;

pub use command::{Command, CommandError};
pub use parser::{Parser, ParserError, Primitive, Tree};
pub use scanner::{Scanner, ScannerError, Token};

use std::{fs, io};

pub fn run(command: Command) -> Result<(), Failure> {
    match command {
        Command::Help => {
            println!("Help is coming (soon)!")
        }
        Command::Play => todo!("set up playground"),
        Command::Parse { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::File)?;

            let scanner = Scanner::new(&input);

            let tokens = scanner.finish().map_err(Failure::Scanner)?;

            let mut parser = Parser::new(tokens.into_iter());

            let tree = parser.parse().map_err(Failure::Parser)?;

            println!("{tree}")
        }
        Command::Tokenize { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::File)?;

            let scanner = Scanner::new(&input);
            let mut errors = Vec::new();

            for next in scanner {
                match next {
                    Ok(token) => println!("{token}"),
                    Err(error) => errors.push(error),
                }
            }

            if !errors.is_empty() {
                return Err(Failure::Scanner(errors));
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Failure {
    File(io::Error),
    Parser(Vec<ParserError>),
    Scanner(Vec<ScannerError>),
}
