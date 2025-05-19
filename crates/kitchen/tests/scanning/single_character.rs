use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn test() {
    let output = run_command(
        crate::BINARY,
        &[
            "tokenize",
            create_temporary_file("({*.,+-*};)")
                .path()
                .to_str()
                .unwrap(),
        ],
    );

    output.success().stdout(predicate::eq(indoc! {"
        LEFT_PAREN ( null
        LEFT_BRACE { null
        STAR * null
        DOT . null
        COMMA , null
        PLUS + null
        MINUS - null
        STAR * null
        RIGHT_BRACE } null
        SEMICOLON ; null
        RIGHT_PAREN ) null
        EOF  null
    "}));
}
