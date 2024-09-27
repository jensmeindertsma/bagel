use bagel::{Command, Failure};
use owo_colors::OwoColorize;
use std::{env, error::Error, process::ExitCode};

pub fn main() -> ExitCode {
    let command = match Command::try_from_iterator(env::args().skip(1)) {
        Ok(command) => command,
        Err(error) => {
            print_error(error);
            return ExitCode::FAILURE;
        }
    };

    if let Err(failure) = bagel::run(command) {
        match failure {
            Failure::File(io_error) => {
                print_error(io_error);
                return ExitCode::FAILURE;
            }
            Failure::Parser(errors) => {
                for error in errors {
                    eprintln!("{error}")
                }
                return ExitCode::from(65);
            }
            Failure::Scanner(errors) => {
                for error in errors {
                    eprintln!("{error}")
                }
                return ExitCode::from(65);
            }
        }
    }

    ExitCode::SUCCESS
}

fn print_error(error: impl Error) {
    println!("{}{} {}", "error".bold().red(), ":".bold(), error.bold())
}
