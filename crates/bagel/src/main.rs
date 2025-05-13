mod codecrafters;
mod colors;

use codecrafters::is_codecrafters_environment;
use colors::Colorization;
use core::fmt::{self, Formatter};
use owo_colors::OwoColorize;
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
        let _is_codecrafters = is_codecrafters_environment();

        // TODO: based on is_codecrafters, print errors differently to pass their tests

        match colorization {
            Colorization::Disabled => eprintln!("error: {failure}"),
            Colorization::Enabled => eprintln!("{}{} {failure}", "error".bold().red(), ":".bold()),
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

    let command = arguments
        .next()
        .ok_or(Failure::MissingArgument("command"))?;

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

            let _contents = fs::read_to_string(path).map_err(Failure::FileRead)?;

            todo!()
        }
        _ => return Err(Failure::UnknownCommand(command)),
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    FileRead(io::Error),
    LogFileCreation(io::Error),
    MissingArgument(&'static str),
    UnknownCommand(String),
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileRead(io_error) => write!(formatter, "failed to read source file: {io_error}"),
            Self::LogFileCreation(io_error) => write!(
                formatter,
                "failed to create log file at `/tmp/bagel.log`: {io_error}"
            ),
            Self::MissingArgument(argument) => write!(
                formatter,
                "missing required argument `{argument}`, run `bagel help` for assistence"
            ),

            Self::UnknownCommand(command) => write!(
                formatter,
                "unknown command `{command}`, run `bagel help` for assistence"
            ),
        }
    }
}
