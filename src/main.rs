mod command;
mod parser;
mod scanner;

use command::{Command, CommandError};
use owo_colors::OwoColorize;
use parser::Parser;
use scanner::Scanner;
use std::{
    env, fs, io,
    process::{ExitCode, Termination},
};

pub fn main() -> impl Termination {
    let failure = match run(env::args().skip(1)) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(failure) => failure,
    };

    let (error, exit_code) = match failure {
        Failure::Command(command_error) => (Some(command_error.to_string()), ExitCode::FAILURE),
        Failure::Io(io_error) => (Some(io_error.to_string()), ExitCode::FAILURE),
        Failure::Silent(exit_code) => (None, exit_code),
    };

    if let Some(message) = error {
        eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message.bold());
    }

    exit_code
}

fn run(arguments: impl Iterator<Item = String>) -> Result<(), Failure> {
    let command = Command::from_arguments(arguments).map_err(Failure::Command)?;

    match command {
        Command::Help => {
            println!("Help is coming (soon)!")
        }

        Command::Tokenize { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let mut failed = false;

            for result in Scanner::new(&input) {
                match result {
                    Ok(token) => println!("{token}"),
                    Err(error) => {
                        failed = true;
                        eprintln!("{error}")
                    }
                }
            }

            if failed {
                return Err(Failure::Silent(ExitCode::from(65)));
            }
        }

        Command::Parse { filename } => {
            let input = fs::read_to_string(filename).map_err(Failure::Io)?;

            let mut tokens = Vec::new();
            let mut failed = false;

            for result in Scanner::new(&input) {
                match result {
                    Ok(token) => tokens.push(token),
                    Err(error) => {
                        failed = true;
                        eprintln!("{error}")
                    }
                }
            }

            if failed {
                return Err(Failure::Silent(ExitCode::from(65)));
            }

            let mut parser = Parser::new(tokens);

            match parser.parse() {
                Ok(tree) => println!("{tree}"),
                Err(error) => {
                    // TODO multiple errors
                    eprintln!("{error}");
                    return Err(Failure::Silent(ExitCode::from(65)));
                }
            }
        }

        Command::Play { input } => {
            let mut tokens = Vec::new();

            println!("{}", "=== TOKENIZATION ===".bold());
            for result in Scanner::new(&input) {
                match result {
                    Ok(token) => {
                        println!("{token}");
                        tokens.push(token)
                    }
                    Err(error) => {
                        eprintln!("{error}")
                    }
                }
            }

            println!("{}", "=== PARSING ===".bold());

            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(tree) => println!("{tree}"),
                Err(error) => {
                    eprintln!("{error}");
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    Command(CommandError),
    Io(io::Error),
    Silent(ExitCode),
}
