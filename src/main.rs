mod program;
mod tracing;

use owo_colors::OwoColorize;
use program::{Failure, ProgramError};
use std::{
    env,
    process::{ExitCode, Termination},
};
use tracing::setup_tracing;

pub fn main() -> impl Termination {
    setup_tracing();

    let show_colors = !matches!(env::var("NO_COLORS"), Ok(value) if value == "true");

    let failure = match program::run(env::args().skip(1)) {
        Ok(_) => return ExitCode::SUCCESS,
        Err(failure) => failure,
    };

    let (error, exit_code) = match failure {
        Failure::InvalidCommand(error) => (Some(error.to_string()), ExitCode::FAILURE),
        Failure::Io(error) => (Some(error.to_string()), ExitCode::FAILURE),
        Failure::Program(error) => match error {
            ProgramError::Scanner(errors) => {
                for error in errors {
                    // Ideally I would format the error nicely, but CodeCrafters
                    // test case does not allow for this: we have to print directly.
                    eprintln!("{error}")
                }

                return ExitCode::from(65);
            }
            ProgramError::Parser(error) => {
                // Ideally I would format the error nicely, but CodeCrafters
                // test case does not allow for this: we have to print directly.
                // (error, ExitCode::from(65))

                eprintln!("{error}");

                return ExitCode::from(65);
            }
            ProgramError::Interpreter(error) => {
                // Ideally I would format the error nicely, but CodeCrafters
                // test case does not allow for this: we have to print directly.
                // (error, ExitCode::from(70))

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
