mod environment;
mod scanner;

use core::fmt::{self, Formatter};
use environment::{Colorization, Environment};
use owo_colors::OwoColorize;
use scanner::{Scanner, ScannerError};
use std::{
    env,
    fmt::Display,
    fs::{self, File},
    io::{self, Write},
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
                            _ => print_error(&failure, colorization),
                        }
                    }
                }

                // Print other errors as normal despite the environment
                // (these errors are not tested on their output).
                other => print_error(other, colorization),
            }
        } else {
            // Print according to the `Display` implementation.
            print_error(&failure, colorization)
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

    match arguments.next() {
        None => start_playground(colorization),
        Some(command) => return handle_command(command, arguments),
    }
}

fn start_playground(colorization: Colorization) -> Result<(), Failure> {
    println!(
        "{}",
        "Welcome to the interactive playground".bold().underline()
    );
    println!(
        "{} {}",
        "->".blue(),
        "run a command with arguments to get output"
    );

    let mut input = String::new();
    loop {
        print!("{} ", ">".bold());

        io::stdout().flush().expect("flushing should work");

        io::stdin()
            .read_line(&mut input)
            .expect("reading input should succeed");

        if input.is_empty() {
            continue;
        }

        if matches!(input.as_str(), "quit" | "exit") {
            return Ok(());
        }

        let mut arguments = input.split_whitespace();

        let command = arguments.next().expect("arguments should not be empty");
        match command {
            "scan" | "tokenize" => {
                let Some(text) = arguments.next() else {
                    print_error("you must provide input to the scanner", colorization);

                    input.clear();
                    continue;
                };

                match scan(text) {
                    Err(errors) => {
                        eprintln!("Encountered errors:");
                        for error in errors {
                            eprintln!("1. {error}")
                        }
                    }
                };
            }
            _ => print_error(Failure::UnknownCommand(command.to_owned()), colorization),
        }

        input.clear();
    }
}

fn handle_command(
    command: String,
    mut arguments: impl Iterator<Item = String>,
) -> Result<(), Failure> {
    match command.as_str() {
        // I prefer `scan` but CodeCrafters expects `tokenize`.
        "scan" | "tokenize" => {
            let path = arguments.next().ok_or(Failure::MissingArgument("file"))?;

            let contents =
                fs::read_to_string(&path).map_err(|error| Failure::Read { path, error })?;

            if let Err(errors) = scan(&contents) {
                return Err(Failure::Scanner(errors));
            };
        }
        _ => {
            return Err(Failure::UnknownCommand(command));
        }
    };

    Ok(())
}

fn scan(input: &str) -> Result<(), Vec<ScannerError>> {
    let scanner = Scanner::new(&input);
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
        return Err(errors);
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

            Self::MissingArgument(argument) => {
                write!(formatter, "missing required argument `{argument}`")
            }

            Self::Scanner(errors) => {
                for error in errors {
                    writeln!(formatter, "{error}")?;
                }

                Ok(())
            }

            Self::UnknownCommand(command) => write!(formatter, "unknown command `{command}`"),
        }
    }
}

fn print_error(error: impl Display, colorization: Colorization) {
    match colorization {
        Colorization::Disabled => eprintln!("error: {error}"),
        Colorization::Enabled => {
            eprintln!("{}{} {error}", "error".bold().red(), ":".bold())
        }
    }
}
