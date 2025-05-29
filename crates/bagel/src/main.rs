mod environment;
mod scanner;

use core::fmt::{self, Formatter};
use environment::{Colorization, Environment};
use owo_colors::OwoColorize;
use scanner::{Scanner, ScannerError};
use std::{
    env,
    fs::{self, File},
    io,
    process::{ExitCode, Termination},
};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(File::create("/tmp/bagel.log").expect("writer should be initialized"))
        .init();

    let (environment, colorization) = Environment::determine();

    if let Err(failure) = run(env::args().skip(1), colorization) {
        // See `environment.rs` for the more details on why we are outputting some errors
        // in a different format than normal when we are running in a testing environment
        // (be it CodeCrafters or running `kitchen`'s integration tests locally).
        if environment == Environment::Testing {
            match &failure {
                Failure::Scanner(errors) => {
                    for error in errors {
                        match error {
                            ScannerError::UnexpectedCharacter { character, line } => {
                                eprintln!("[line {line}] Error: Unexpected character: {character}")
                            }
                            ScannerError::UnexpectedEndOfFile { line } => {
                                eprintln!("[line {line}] Error: Unterminated string.")
                            }
                        }
                    }
                }

                // Print other errors as normal despite the environment
                // (these errors are not tested on their output).
                other => print_failure(other, colorization),
            }
        } else {
            // Print according to the `Display` implementation.
            print_failure(&failure, colorization)
        }

        failure.exit_code()
    } else {
        ExitCode::SUCCESS
    }
}

fn run(
    arguments: impl IntoIterator<Item = String>,
    colorization: Colorization,
) -> Result<(), Failure> {
    // The first argument is the binary name/path which we can ignore.
    let mut arguments = arguments.into_iter();

    let Some(command) = arguments.next() else {
        todo!("implement REPL");
    };

    match command.as_str() {
        "help" => {
            let header = "HELP - COMMAND OVERVIEW";
            match colorization {
                Colorization::Disabled => println!("{header}"),
                Colorization::Enabled => println!("{}", header.bold().underline()),
            }

            for (command, arguments) in [("scan", "{file}")] {
                match colorization {
                    Colorization::Disabled => println!("* `bagel {command} {arguments}`"),
                    Colorization::Enabled => println!(
                        "{} `{} {}`",
                        "*".bold().red(),
                        format!("bagel {command}").bold(),
                        arguments.italic()
                    ),
                }
            }
        }

        // I prefer `scan` but CodeCrafters expects `tokenize`.
        "scan" | "tokenize" => {
            let path = arguments.next().ok_or(Failure::MissingArgument("file"))?;

            let contents =
                fs::read_to_string(&path).map_err(|error| Failure::Read { path, error })?;

            let scanner = Scanner::new(&contents);
            let mut errors = Vec::new();

            for result in scanner {
                match result {
                    Ok(token) => {
                        println!("{token}");
                    }
                    Err(error) => {
                        errors.push(error);
                    }
                }
            }

            if !errors.is_empty() {
                return Err(Failure::Scanner(errors));
            }
        }
        _ => {
            return Err(Failure::UnknownCommand(command));
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    Read { path: String, error: io::Error },
    MissingArgument(&'static str),
    Scanner(Vec<ScannerError>),
    UnknownCommand(String),
}

impl Failure {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Scanner(_) => ExitCode::from(65),
            _ => ExitCode::FAILURE,
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, error } => {
                write!(formatter, "failed to read source file `{path}`: {error}")
            }

            Self::MissingArgument(argument) => write!(
                formatter,
                "missing required argument `{argument}`, run `bagel help` for assistence"
            ),

            Self::Scanner(errors) => {
                for error in errors {
                    writeln!(formatter, "{error}")?;
                }

                Ok(())
            }

            Self::UnknownCommand(command) => write!(
                formatter,
                "unknown command `{command}`, run `bagel help` for assistence"
            ),
        }
    }
}

fn print_failure(failure: &Failure, colorization: Colorization) {
    match colorization {
        Colorization::Disabled => eprintln!("error: {failure}"),
        Colorization::Enabled => {
            eprintln!("{}{} {failure}", "error".bold().red(), ":".bold())
        }
    }
}
