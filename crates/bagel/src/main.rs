mod environment;
mod interpreter;
mod reporter;

use environment::Environment;
use reporter::{BasicPrinter, FancyPrinter, Reporter};
use std::{
    env,
    process::{ExitCode, Termination},
};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let environment = Environment::determine();

    if let Err(error) = interpreter::run(env::args().skip(1)) {
        match environment {
            Environment::Testing => BasicPrinter::report(&error),
            Environment::Regular => FancyPrinter::report(&error),
        }

        error.exit_code()
    } else {
        ExitCode::SUCCESS
    }
}
