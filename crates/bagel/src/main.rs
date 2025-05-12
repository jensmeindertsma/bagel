mod colors;

use colors::Colorization;
use core::fmt::{self, Formatter};
use owo_colors::OwoColorize;
use std::{
    env,
    fs::File,
    io,
    process::{ExitCode, Termination},
};
use tracing::Level;

fn main() -> impl Termination {
    let colorization = Colorization::determine();

    if let Err(failure) = run(colorization) {
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
    let arguments = env::args().skip(1);

    let header = format!("received {} arguments:", arguments.len());
    match colorization {
        Colorization::Disabled => println!("{header}"),
        Colorization::Enabled => println!("{}", header.bold()),
    };

    for argument in arguments {
        match colorization {
            Colorization::Disabled => println!("- `{argument}`"),
            Colorization::Enabled => println!("{} `{}`", "-".bold(), argument),
        };
    }

    Ok(())
}

#[derive(Debug)]
enum Failure {
    LogFileCreation(io::Error),
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogFileCreation(io_error) => write!(
                formatter,
                "failed to create log file at `/tmp/bagel.log`: {io_error}"
            ),
        }
    }
}
