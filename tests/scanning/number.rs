use crate::support::{setup_command_environment, multiline_output};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn integer() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "42";

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(multiline_output(
        "
        NUMBER 42 42.0
        EOF  null
        ",
    )));
}

#[test]
fn decimal() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "1234.1234").unwrap();

    cmd.assert().success().stdout(predicate::eq(multiline_output(
        "
        NUMBER 1234.1234 1234.1234
        EOF  null
        ",
    )));
}
