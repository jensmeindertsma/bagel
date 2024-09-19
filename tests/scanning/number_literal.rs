use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn integer() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "42";

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
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

    let contents = "1234.1234";

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        NUMBER 1234.1234 1234.1234
        EOF  null
        ",
    )));
}
