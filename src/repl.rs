use crate::interpreter::Interpreter;
use owo_colors::OwoColorize;
use std::io::{BufRead, Write};
use std::{env, io};

const PROMPT: &str = "《🥯》";

fn print_info(message: &str) {
    println!("{}{}", PROMPT.bold().blue(), message.bold().blue());
}

fn print_error(message: &str) {
    eprintln!("{}{}", PROMPT.bold().red(), message.bold().red());
}

pub fn start_interactive_shell() {
    println!(
        "{}",
        format!(
            "Welcome, thanks for using Bagel v{}",
            env!("CARGO_PKG_VERSION")
        )
        .bold()
        .yellow(),
    );

    let mut buffer = String::new();

    loop {
        print!("{}", PROMPT.bold().green());
        io::stdout().flush().unwrap();

        if io::stdin().lock().read_line(&mut buffer).is_err() {
            print_error("didn't quite get that, please try again");
            buffer.clear();
            continue;
        };

        let input = buffer.trim();

        if input == "quit" {
            print_info("quitting...");
            return;
        }

        // TODO: properly support multi-line

        let interpreter = Interpreter::default();

        let output = interpreter.interpret(input);

        match output {
            Ok(output) => print_info(&output),
            Err(interpreter_error) => print_error(&format!("error: {interpreter_error:?}")),
        }

        buffer.clear()
    }
}
