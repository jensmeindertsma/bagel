mod command;
mod interpreter;
mod parser;
mod printer;
mod scanner;
mod token;
mod tree;

use command::{Command, CommandError};
use interpreter::{Interpreter, InterpreterError};
use parser::{Parser, ParserError};
use printer::Printer;
use scanner::{Scanner, ScannerError};
use std::{fs, io};
use tracing::instrument;

#[instrument(name = "program", skip_all)]
pub fn run(arguments: impl Iterator<Item = String>) -> Result<(), Failure> {
    let command = Command::from_arguments(arguments).map_err(Failure::InvalidCommand)?;

    match command {
        Command::Help => {
            println!("Help is coming (soon)!")
        }

        Command::Tokenize { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            tracing::info!("read input: `{input}`");

            match Scanner::new(&input).scan() {
                Ok(tokens) => {
                    for token in tokens {
                        println!("{}", token.kind)
                    }
                }
                Err(errors) => {
                    return Err(Failure::Program(ProgramError::Scanner(errors)));
                }
            };
        }

        Command::Parse { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            tracing::info!("read input: `{input}`");

            let tokens = Scanner::new(&input)
                .scan()
                .map_err(|e| Failure::Program(ProgramError::Scanner(e)))?;

            let mut parser = Parser::new(tokens);

            let tree = parser
                .parse()
                .map_err(|e| Failure::Program(ProgramError::Parser(e)))?;

            println!("{}", Printer::new(&tree).print());
        }

        Command::Evaluate { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let tokens = Scanner::new(&input)
                .scan()
                .map_err(|e| Failure::Program(ProgramError::Scanner(e)))?;

            let mut parser = Parser::new(tokens);

            let tree = parser
                .parse()
                .map_err(|e| Failure::Program(ProgramError::Parser(e)))?;

            let interpreter = Interpreter::new(tree);

            let value = interpreter
                .evaluate()
                .map_err(|e| Failure::Program(ProgramError::Interpreter(e)))?;

            println!("{value}");
        }

        Command::Run { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let tokens = Scanner::new(&input)
                .scan()
                .map_err(|e| Failure::Program(ProgramError::Scanner(e)))?;

            let mut parser = Parser::new(tokens);

            let tree = parser
                .parse()
                .map_err(|e| Failure::Program(ProgramError::Parser(e)))?;

            let interpreter = Interpreter::new(tree);

            interpreter
                .run()
                .map_err(|e| Failure::Program(ProgramError::Interpreter(e)))?;
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
    Scanner(Vec<ScannerError>),
    Parser(ParserError),
    Interpreter(InterpreterError),
}
