use crate::support::{format_expected_output, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn single_char_tokens() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "({*.,+-*};)";

    write!(file, "{contents}").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(format_expected_output(
            "
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
            ",
        )));
}
