mod playground;
mod scanner;

use playground::Playground;
use scanner::Scanner;
use std::{
    env,
    fs::{self, File},
    process::Termination,
};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(File::create("/tmp/bagel.log").expect("file creation should succeed"))
        .init();

    let mut arguments = env::args().skip(1);

    match arguments.next() {
        None => {
            Playground::initialize().start();
        }
        Some(command) => match command.as_str() {
            "scan" | "tokenize" => {
                let input = fs::read_to_string(arguments.next().unwrap()).unwrap();

                for output in Scanner::new(&input) {
                    match output {
                        Ok(token) => println!("{token}"),
                        Err(_error) => {
                            // if is_codecrafters() {
                            //     eprintln!("[line 1] Error: Unexpected character: $")
                            // } else {
                            //     // Figure out printing reporting erros regularly and also exit code
                            // }
                            panic!("scanner error!")
                        }
                    }
                }
            }
            "parse" => {}
            _ => panic!(),
        },
    }
}
