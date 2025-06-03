// use std::{
//     error::Error,
//     fmt::Display,
//     io::{self, Read, Write},
// };

// use owo_colors::OwoColorize;

// pub struct Playground {
//     buffer: String,
// }

// impl Playground {
//     pub fn initialize() -> Self {
//         Self {
//             buffer: String::new(),
//         }
//     }

//     pub fn start(&mut self) {
//         loop {
//             let input = self.prompt();

//             let mut parts = input.split_whitespace();

//             if let Some(command) = parts.next() {
//                 println!("command = {command}")
//             } else {
//                 self.print_error("a command is required (try `help`)");
//             }
//         }
//     }

//     fn prompt(&mut self) -> &str {
//         print!("{} ", ">".bold());
//         io::stdout().flush().unwrap();

//         self.buffer.clear();
//         io::stdin().read_to_string(&mut self.buffer).unwrap();

//         self.buffer.trim()
//     }

//     fn print_error(&mut self, error: &str) {
//         eprintln!("{}{} {}", "error".bold().red(), ":".bold(), error.bold());
//     }
// }

// #[derive(Debug)]
// pub enum PlaygroundError {}

// impl Display for PlaygroundError {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "todo")
//     }
// }

// impl Error for PlaygroundError {}

// // pub fn handle_input(input: &str) -> Result<(), PlaygroundError> {
// //     let input = input.trim();

// //     let mut parts = input.split_whitespace();

// //     if let Some(command) = parts.next() {
// //         match command {
// //             "exit" | "quit" => {
// //                 println!("{}", "Shutting down, see you next time!".bold());

// //                 return;
// //             }
// //             "help" => {
// //                 println!("{}", "HELP".bold().green().underline());

// //                 for (command, arguments) in [("help", None), ("scan", Some("input"))] {
// //                     if let Some(arguments) = arguments {
// //                         println!(
// //                             "{} {} {}",
// //                             "-".bold().red(),
// //                             command.bold(),
// //                             arguments.italic()
// //                         );
// //                     } else {
// //                         println!("{} {}", "-".bold().red(), command.bold(),);
// //                     }
// //                 }
// //             }
// //             _ => todo!(),
// //         }
// //     } else {
// //     }
// // }
