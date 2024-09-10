use std::io::{BufRead, Write};
use std::process::ExitCode;
use std::{env, io};

use owo_colors::OwoColorize;

const PROMPT: &str = "《🥯》";

fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);

    let command = arguments.next();

    if command.is_none() {
        println!(
            "{}",
            format!(
                "Welcome, thanks for using Bagel v{}",
                env!("CARGO_PKG_VERSION")
            )
            .bold()
            .yellow(),
        );

        // TODO: before starting REPL, check if there is already source being fed into standard input,
        // handle that instead if it is the case.

        let mut buffer = String::new();

        loop {
            print!("{}", PROMPT.bold().green());
            io::stdout().flush().unwrap();

            if io::stdin().lock().read_line(&mut buffer).is_ok() {
                print_error("please try again");
                continue;
            };

            let input = buffer.trim();

            print_output(input);

            // TODO: first check for any repl-related input commands:
            // - quit, multi line stuff?
            // send off input to interpreter here, figure out a way to allow
            // multiple statements to be entered on one line (semicolon?
            // or backslash ? or custom command to enter multi line, or use of EOF)

            buffer.clear()
        }
    }

    // TODO: parse and handle different commands.

    ExitCode::SUCCESS
}

fn print_output(output: &str) {
    println!("{}{}", PROMPT.bold().blue(), output.bold().blue());
}

fn print_error(message: &str) {
    eprintln!("{}{}", PROMPT.bold().red(), message.bold().red());
}
