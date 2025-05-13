mod codecrafters;
mod colors;
mod scanner;

use codecrafters::is_codecrafters_environment;
use colors::Colorization;
use core::fmt::{self, Formatter};
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
    let colorization = Colorization::determine();

    if let Err(failure) = run(colorization) {
        match failure {
            Failure::Scanner(errors) => {
                if is_codecrafters_environment() {
                    for error in errors {
                        match error {
                            ScannerError::UnexpectedCharacter { character, line } => {
                                eprintln!("[line {line}] Error: Unexpected character: {character}")
                            }
                        }
                    }
                } else {
                    print_failure(Failure::Scanner(errors), colorization);
                }
            }
            failure => print_failure(failure, colorization),
        }

        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run(colorization: Colorization) -> Result<(), Failure> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(File::create("/tmp/bagel.log").map_err(Failure::LogFileCreation)?)
        .init();

    // the first argument is the binary which we can ignore
    let mut arguments = env::args().skip(1);

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
            let path = arguments.next().ok_or(Failure::MissingArgument("file"))?;

            let contents =
                fs::read_to_string(&path).map_err(|error| Failure::FileRead { path, error })?;

            let scanner = Scanner::new(&contents);
            let mut errors = Vec::new();

            for result in scanner {
                match result {
                    Ok(token) => println!("{token}"),
                    Err(error) => errors.push(error),
                }
            }

            if !errors.is_empty() {
                return Err(Failure::Scanner(errors));
            }
        }
        _ => return Err(Failure::UnknownCommand(command)),
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
        ExitCode::FAILURE
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

fn print_failure(failure: Failure, colorization: Colorization) {
    match colorization {
        Colorization::Disabled => eprintln!("error: {failure}"),
        Colorization::Enabled => {
            eprintln!("{}{} {failure}", "error".bold().red(), ":".bold())
        }
    }
}
