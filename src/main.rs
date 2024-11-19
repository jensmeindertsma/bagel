mod parser;
mod scanner;

use owo_colors::OwoColorize;
use scanner::Scanner;
use std::{env, fs, io, process::ExitCode};

fn print_error(error: impl std::error::Error) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), error.bold());
}

pub fn main() -> ExitCode {
    if let Err(error) = run(env::args().skip(1)) {
        match error {
            Error::Scanner => ExitCode::from(65),
            Error::MissingCommand => {}
        }
    } else {
        ExitCode::SUCCESS
    }
}

fn run(mut arguments: impl Iterator<Item = String>) -> Result<(), CommandError> {
    match arguments
        .next()
        .ok_or(CommandError::MissingCommand)?
        .as_str()
    {
        "help" => {
            println!("Help is coming (soon)!")
        }

        "tokenize" => {
            let filename = arguments.next().ok_or(Error::MissingFilename)?;
            let input = fs::read_to_string(filename).map_err(CommandError::Io)?;

            let mut scanner = Scanner::new(&input);
            let mut failed = false;

            for result in scanner {
                match result {
                    Ok(token) => println!("{token}"),
                    Err(error) => {
                        failed = true;
                        eprintln!("{error}")
                    }
                }
            }

            if failed {
                return Err(CommandError::Scanner);
            }
        }

        unknown => return Err(CommandError::UnknownCommand(unknown.to_owned())),
    }

    Ok(())
}

#[derive(Debug)]
enum CommandError {
    Io(io::Error),
    MissingCommand,
    MissingFilename,
    Scanner,
    UnknownCommand(String),
}
