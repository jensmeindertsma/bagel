mod support;

use predicates::prelude::*;
use std::{fs::File, io::Write};
use support::{format_expected_output, setup_command_environment};

#[test]
fn empty_file() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    File::create(temp_dir.join("test.lox")).unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(format_expected_output(
            "
            EOF  null
        ",
        )));
}

#[test]
fn parentheses() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(()").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(format_expected_output(
            "
            LEFT_PAREN ( null
            LEFT_PAREN ( null
            RIGHT_PAREN ) null
            EOF  null
        ",
        )));
}
