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

mod scanner;

use std::process::{ExitCode, Termination};
use tracing::Level;

fn main() -> impl Termination {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(create_log_file())
        .init();

    let environment = Environment::determine();

    if let Err(failure) = run(env::args().skip(1)) {
        return failure.exit_code();
    }

    ExitCode::SUCCESS
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<(), Failure> {
    let _arguments = arguments.into_iter();

    Ok(())
}

enum Failure {}
