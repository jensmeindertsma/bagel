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
                            _ => todo!(),
                        }
                    }
                }
                // Print errors that don't require special formatting as normal
                // (this would include errors that are not tested on their output).
                other => print_failure(other, colorization),
            }
        } else {
            // When we are not running in a testing environment we print
            // errors like usual (as defined in their `Display` implementation).
            print_failure(&failure, colorization)
        }

        // The exit code on termination is determined by the kind failure.
        // See the method implementation for the specific exit codes.
        failure.exit_code()
    } else {
        ExitCode::SUCCESS
    }
}

fn run(
    arguments: impl IntoIterator<Item = String>,
    colorization: Colorization,
) -> Result<(), Failure> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(File::create("/tmp/bagel.log").map_err(Failure::LogFileCreation)?)
        .init();

    // The first argument is the binary name/path which we can ignore.
    let mut arguments = arguments.into_iter();

    let Some(command) = arguments.next() else {
        todo!("implement REPL");
    };

    match command.as_str() {
        "help" => {
            tracing::info!("running command `help`");

            let header = "HELP - COMMAND OVERVIEW";
            match colorization {
                Colorization::Disabled => println!("{header}"),
                Colorization::Enabled => println!("{}", header.bold().underline()),
            }

            for (command, arguments) in [("tokenize", "{file}")] {
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

        "tokenize" => {
            tracing::info!("running command `tokenize`");

            let path = arguments.next().ok_or(Failure::MissingArgument("file"))?;

            let contents = match fs::read_to_string(&path) {
                Ok(contents) => {
                    tracing::trace!(
                        "read {} characters from the file at `{}`",
                        contents.len(),
                        path
                    );

                    contents
                }
                Err(error) => return Err(Failure::FileRead { path, error }),
            };

            let scanner = Scanner::new(&contents);
            let mut errors = Vec::new();

            for result in scanner {
                //tracing::trace!("scanner produced {result:?}");
                match result {
                    Ok(token) => {
                        println!("{token}")
                    }
                    Err(error) => errors.push(error),
                }
            }

            if !errors.is_empty() {
                return Err(Failure::Scanner(errors));
            }
        }
        _ => {
            tracing::error!("unknown command `{}`", command);

            return Err(Failure::UnknownCommand(command));
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    FileRead { path: String, error: io::Error },
    LogFileCreation(io::Error),
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
            Self::FileRead { path, error } => {
                write!(formatter, "failed to read source file `{path}`: {error}")
            }

            Self::LogFileCreation(io_error) => write!(
                formatter,
                "failed to create log file at `/tmp/bagel.log`: {io_error}"
            ),

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
