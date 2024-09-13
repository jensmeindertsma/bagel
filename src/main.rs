mod command;
mod interpreter;
mod repl;

use command::{Command, CommandParseError, TryFromIterator};
use interpreter::{Interpreter, InterpreterError, ScannerError};
use owo_colors::OwoColorize;
use std::io::{IsTerminal, Read};
use std::process::{self, ExitCode};
use std::{env, fs, io};

fn main() -> ExitCode {
    let arguments = env::args().skip(1);

    if arguments.len() < 1 {
        let stdin_is_pipe = !io::stdin().is_terminal();
        if stdin_is_pipe {
            return handle_piped_stdin();
        }

        // Without any arguments, we boot up the REPL.
        repl::start_interactive_shell();
        return ExitCode::SUCCESS;
    }

    let command = match Command::try_from_iterator(arguments) {
        Ok(c) => c,
        Err(error) => {
            print_error(
                &match error {
                    CommandParseError::MissingCommand => "missing command".to_owned(),
                    CommandParseError::MissingFilename => "missing filename".to_owned(),
                    CommandParseError::UnknownCommand(command) => {
                        format!("unknown command `{command}`")
                    }
                },
                Color::On,
            );
            return ExitCode::FAILURE;
        }
    };

    match command {
        Command::Help => {
            println!("{}", "HELP IS COMING!!!!".bold().underline())
        }
        Command::Tokenize { filename } => {
            let contents = match fs::read_to_string(filename) {
                Ok(c) => c,
                Err(error) => {
                    let message = match error.kind() {
                        io::ErrorKind::NotFound => "file not found",
                        _ => "unknown error reading file",
                    };

                    print_error(message, Color::On);
                    return ExitCode::FAILURE;
                }
            };

            let interpreter = Interpreter::default();
            let tokens = match interpreter.scan(&contents) {
                Ok(tokens) => tokens,
                Err(error) => {
                    print_error(&format!("scanner failure: {error:?}"), Color::On);
                    return ExitCode::FAILURE;
                }
            };

            for token in tokens {
                println!("{token}")
            }
        }
    }

    ExitCode::SUCCESS
}

#[derive(PartialEq)]
enum Color {
    On,
    Off,
}

fn handle_piped_stdin() -> process::ExitCode {
    let do_color_output = match io::stdout().is_terminal() {
        true => Color::On,
        false => Color::Off,
    };

    let mut buffer = Vec::new();
    if io::stdin().read_to_end(&mut buffer).is_err() {
        print_error("failed to read from standard input", do_color_output);
        return ExitCode::FAILURE;
    };

    let input = match String::from_utf8(buffer) {
        Ok(string) => string,
        Err(_) => {
            print_error(
                "received invalid UTF-8 over standard input",
                do_color_output,
            );
            return ExitCode::FAILURE;
        }
    };

    let interpreter = Interpreter::default();
    let output = interpreter.interpret(&input);

    match output {
        Ok(result) => println!("{result}"),
        Err(interpreter_error) => {
            print_error(
                &match interpreter_error {
                    InterpreterError::Scanner(scanner_error) => match scanner_error {
                        ScannerError::UnknownCharacter(c) => {
                            format!("scanner encountered unknown character `{c}`")
                        }
                    },
                },
                do_color_output,
            );
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

fn print_error(message: &str, enable_color: Color) {
    if enable_color == Color::On {
        println!("{}{} {}", "error".bold().red(), ":".bold(), message.bold())
    } else {
        println!("error: {}", message)
    }
}
