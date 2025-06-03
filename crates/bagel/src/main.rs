mod scanner;

use chrono::Local;
use owo_colors::OwoColorize;
use scanner::Scanner;
use std::{
    env,
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, Write},
    process::{ExitCode, Termination},
};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(create_log_file())
        .init();

    let environment = Environment::determine();

    if let Err(failure) = run(env::args()) {
        // TODO: The biggest problem I'm running into is ERROR HANDLING
        // I have to show different simplified error messages for SOME error
        // types when running under a testing environment. I also want to
        // show pretty errors for all other cases. This means I want to show
        // the source code, the line above and below the error, the erroneous
        // position or span colorized RED.

        // 3 | print("cool")
        // 4 | let x = F23;
        // ! |         ^ unexpected token `F`
        // 5 | if x > 5 then

        // [line 4] Unexpected character: F

        // - What about multi line errors?
        // - What if the scanner finds multiple errors?
        //      - Do we stop on first error and print error?
        //      - or do we continue and aggragate all errors and then show a list?

        // > Note also that we keep scanning.
        // > There may be other errors later in the program.
        // > It gives our users a better experience if we detect
        // > as many of those as possible in one go. Otherwise,
        // > they see one tiny error and fix it, only to have the
        // > next error appear, and so on. Syntax error Whac-A-Mole is no fun.

        // > The code reports each invalid character separately, so this shotguns
        // > the user with a blast of errors if they accidentally paste a big blob
        // > of weird text. Coalescing a run of invalid characters into a single
        // > error would give a nicer user experience.

        // Scanner encountered multiple errors:
        //
        // 3  | print("cool")
        // 4  | let x = F23;
        // !  |         ^^^
        // 5  | if x > 5 then
        //
        // 24 | while X < 4
        //            ^
        //
        // SUMMARY
        // - Line 4
        //  - Unexpected character `F` at position 6
        //  - Unexpected character `2` at position 7
        //  - Unexpected character `3` at position 8

        // 3  | print("cool")
        // 4  | let x = F23;
        // !  |         ^^^
        // 5  | if x > 5 then
        //
        // SUMMARY
        // 1.  unexpected character sequence on line 4 position 6-8
        // 2.  unexpected end of input on line 5 position 9

        // And what about parsing errors? And interpreter errors? How to present
        // errors during the execution of the program?

        report_failure(environment, &failure);

        return failure.exit_code();
    }

    ExitCode::SUCCESS
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<(), Failure> {
    let mut arguments = arguments.into_iter().skip(1);

    match arguments.next() {
        None => {
            println!("{}", "Welcome to the playground!".bold().blue().underline());

            let mut buffer = String::new();

            loop {
                buffer.clear();

                print!("{} ", ">".bold());
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut buffer).unwrap();

                let input = buffer.trim();
                let mut parts = input.split_whitespace();

                match parts.next() {
                    None => {
                        print_error("a command is required (try `help`");
                        continue;
                    }
                    Some(command) => match command {
                        "exit" | "quit" => {
                            println!();
                            println!("{}", "Goodbye, see you next time!".bold().underline());
                            println!();
                            return Ok(());
                        }

                        "help" => {
                            println!();
                            println!("{}", "HELP".bold().green().underline());
                            println!("{} help", "-".bold().red());
                            println!("{} scan {}", "-".bold().red(), "input".italic());
                            println!();
                            continue;
                        }

                        "scan" => {
                            let Some(input) = parts.next() else {
                                print_error("scanner requires `input` argument");
                                continue;
                            };

                            let scanner = Scanner::new(input);

                            println!();
                            println!("{}", "SCANNING".bold().green().underline());

                            for result in scanner {
                                match result {
                                    Ok(token) => {
                                        println!("{} {}", "-".bold().blue(), token.italic())
                                    }
                                    Err(error) => {
                                        eprintln!("{} {}", "!".bold().red(), error.message.italic())
                                    }
                                }
                            }

                            println!()
                        }

                        _ => {
                            print_error(format!("unknown command `{command}` (try `help`)"));
                            println!();
                            continue;
                        }
                    },
                }
            }
        }
        Some(command) => match command.as_str() {
            "scan" | "tokenize" => {}

            _ => return Err(Failure::UnknownCommand(command)),
        },
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Failure {
    UnknownCommand(String),
    Scanner,
}

impl Failure {
    fn exit_code(&self) -> ExitCode {
        match self {
            Self::UnknownCommand(_) => ExitCode::from(2),
            Self::Scanner => ExitCode::from(65),
        }
    }
}

impl Display for Failure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scanner => {
                unimplemented!()
            }
            Self::UnknownCommand(command) => {
                write!(formatter, "unknown command `{command}` (try `bagel help`")
            }
        }
    }
}

impl Error for Failure {}

#[derive(Clone, Copy, Debug)]
enum Environment {
    Regular,
    Testing,
}

impl Environment {
    fn determine() -> Self {
        match env::var("CODECRAFTERS") {
            Ok(value) if value == "yes" => Self::Testing,
            _ => Self::Regular,
        }
    }
}

fn report_failure(environment: Environment, failure: &Failure) {
    // How do we adapt errors only in the right environment AND only the errors that
    // require special treatment? There is no "fall-through" that we can use. We need
    // to print multiple times. Especially when it comes to the scanner
}

fn print_error(error: impl Display) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), error.bold())
}

fn create_log_file() -> File {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d-%H:%M:%S");

    let filename = match env::args().nth(1) {
        Some(argument) => format!("bagel-{timestamp}-{argument}.log"),
        None => format!("bagel-{timestamp}-none.log"),
    };

    File::create(format!("/tmp/{filename}")).unwrap()
}
