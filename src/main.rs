mod command;
mod scanner;

use command::{Command, CommandParseError, TryFromIterator};
use owo_colors::OwoColorize;
use scanner::{Scanner, ScannerError};
use std::process::ExitCode;
use std::{env, fs, io};

fn main() -> ExitCode {
    let arguments = env::args().skip(1);

    let command = match Command::try_from_iterator(arguments) {
        Ok(c) => c,
        Err(error) => {
            print_error(&match error {
                CommandParseError::MissingCommand => "missing command".to_owned(),
                CommandParseError::MissingFilename => "missing filename".to_owned(),
                CommandParseError::UnknownCommand(command) => {
                    format!("unknown command `{command}`")
                }
            });
            return ExitCode::FAILURE;
        }
    };

    match run(command) {
        Ok(_) => ExitCode::SUCCESS,
        Err(failure) => match failure {
            Failure::FileNotFound => ExitCode::from(2),
            Failure::Scanner => ExitCode::from(65),
        },
    }
}

fn run(command: Command) -> Result<(), Failure> {
    match command {
        Command::Help => {
            println!("{}", "HELP IS COMING!!!!".bold().underline())
        }
        Command::Tokenize { filename } => {
            let contents = match fs::read_to_string(filename) {
                Ok(c) => c,
                Err(error) => {
                    let message = match error.kind() {
                        io::ErrorKind::NotFound => "file not found",
                        _ => "unknown error reading file",
                    };

                    print_error(message);
                    return Err(Failure::FileNotFound);
                }
            };

            let mut failure = false;
            let scanner = Scanner::new(&contents);

            for next in scanner {
                match next {
                    Ok(token) => println!("{token}"),
                    Err(error) => {
                        failure = true;
                        match error {
                            ScannerError::UnknownCharacter { character, line } => {
                                eprintln!("[line {line}] Error: Unexpected character: {character}",)
                            }
                            ScannerError::UnterminatedString { line } => {
                                eprintln!("[line {line}] Error: Unterminated string.",)
                            }
                        }
                    }
                }
            }

            if failure {
                return Err(Failure::Scanner);
            }
        }
    }

    Ok(())
}

enum Failure {
    FileNotFound,
    Scanner,
}

fn print_error(message: &str) {
    println!("{}{} {}", "error".bold().red(), ":".bold(), message.bold())
}
