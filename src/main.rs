mod program;

use owo_colors::OwoColorize;
use program::{run, Failure, ProgramError};
use std::{
    env,
    process::{ExitCode, Termination},
};

pub fn main() -> impl Termination {
    let show_colors = !matches!(env::var("NO_COLORS"), Ok(value) if value == "true");

    let failure = match run(env::args().skip(1)) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(failure) => failure,
    };

    let (error, exit_code) = match failure {
        Failure::InvalidCommand(error) => (Some(error.to_string()), ExitCode::FAILURE),
        Failure::Io(error) => (Some(error.to_string()), ExitCode::FAILURE),
        Failure::Program(error) => match error {
            ProgramError::Scanner(errors) => {
                for error in errors {
                    eprintln!("{error}")
                }

                return ExitCode::from(65);
            }
            ProgramError::Parser(error) => {
                eprintln!("{error}");

                return ExitCode::from(65);
            }
            ProgramError::Interpreter(error) => {
                eprintln!("{error}");

                return ExitCode::from(70);
            }
        },
    };

    if let Some(message) = error {
        if show_colors {
            eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message.bold());
        } else {
            eprintln!("error: {message}");
        }
    }

    exit_code
}
