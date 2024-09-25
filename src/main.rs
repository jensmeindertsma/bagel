mod command;
mod parser;
mod scanner;

use command::{Command, TryFromIterator};
use core::fmt::{self, Formatter};
use owo_colors::OwoColorize;
use parser::{Parser, ParserError};
use scanner::{Scanner, ScannerError, Token};
use std::error::Error;
use std::process::ExitCode;
use std::{env, fs, io};

fn main() -> ExitCode {
    let arguments = env::args().skip(1);

    let command = match Command::try_from_iterator(arguments) {
        Ok(c) => c,
        Err(error) => {
            print_error(error);
            return ExitCode::FAILURE;
        }
    };

    match run(command) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => match error {
            Failure::FileReadError(_) => {
                print_error(error);
                ExitCode::from(2)
            }
            Failure::Scanner(errors) => {
                for error in errors {
                    eprintln!("{error}")
                }

                ExitCode::from(65)
            }
        },
    }
}

fn run(command: Command) -> Result<(), Failure> {
    match command {
        Command::Help => {
            println!("{}", "HELP IS COMING!!!!".bold().underline())
        }
        Command::Parse { filename } => {
            let contents = fs::read_to_string(filename).map_err(Failure::FileReadError)?;

            let scanner = Scanner::new(&contents);
            let tokens = scanner.finish().map_err(Failure::Scanner)?;

            let parser = Parser::new(tokens);
            let tree = parser.finish().map_err(Failure::Parser)?;
        }
        Command::Tokenize { filename } => {
            let contents = fs::read_to_string(filename).map_err(Failure::FileReadError)?;

            let mut errors = Vec::new();
            let scanner = Scanner::new(&contents);

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
enum Failure {
    FileReadError(io::Error),
    Parser(ParserError),
    Scanner(Vec<ScannerError>),
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileReadError(io_error) => write!(f, "failed to read file: {io_error}"),
            Self::Parser(error) => write!(f, "parser failed: {error}"),
            Self::Scanner(_) => write!(f, "scanner failed",),
        }
    }
}

impl Error for Failure {}

fn print_error(error: impl Error) {
    println!("{}{} {}", "error".bold().red(), ":".bold(), error.bold())
}
