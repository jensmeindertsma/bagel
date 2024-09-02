use std::env;
use std::fs;
use std::process::ExitCode;

use owo_colors::OwoColorize;

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    let command = match Command::try_from(arguments) {
        Ok(command) => command,
        Err(error) => {
            if let CommandParseError::UnknownCommand(command) = error {
                print_error(&format!("unknown command `{}`", command));
            } else {
                print_error(&format!(
                    "invalid arguments, run `{}` to view supported commands",
                    "bagel help".bold()
                ));
            }
            return ExitCode::FAILURE;
        }
    };

    match command {
        Command::Help => {
            println!("{}", "SUPPORTED COMMANDS".bold().underline());
            println!("{} (prints this message)", "bagel help".italic());
            println!(
                "{} {} (tokenizes input Lox file and prints result to standard output)",
                "bagel tokenize".italic(),
                "<file_name>".bold(),
            );
        }
        Command::Tokenize { filename } => {
            let file_contents = match fs::read_to_string(&filename) {
                Ok(contents) => contents,
                Err(_) => {
                    print_error(&format!("failed to read file `{}`", filename));
                    return ExitCode::FAILURE;
                }
            };

            if file_contents.is_empty() {
                println!("EOF  null")
            }
        }
    }

    ExitCode::SUCCESS
}

fn print_error(message: &str) {
    eprintln!("{}{} {message}", "ERROR".bold().red(), ":".bold(),);
}

#[derive(Debug)]
enum Command {
    Help,
    Tokenize { filename: String },
}

impl TryFrom<Vec<String>> for Command {
    type Error = CommandParseError;

    fn try_from(arguments: Vec<String>) -> Result<Self, Self::Error> {
        let mut arguments = arguments.into_iter();

        let command = arguments.next().ok_or(CommandParseError::Other)?;
        match command.as_str() {
            "help" => Ok(Command::Help),
            "tokenize" => {
                let filename = arguments.next().ok_or(CommandParseError::Other)?;

                Ok(Command::Tokenize { filename })
            }
            _ => Err(CommandParseError::UnknownCommand(command)),
        }
    }
}

enum CommandParseError {
    UnknownCommand(String),
    Other,
}
