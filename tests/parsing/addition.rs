use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn tokenize() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "2 + 3").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        NUMBER 2 2.0
        PLUS + null
        NUMBER 3 3.0
        EOF  null
        ",
    )));
}

#[test]
fn parse() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "2 + 3").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        (+ 2.0 3.0)
        ",
    )));
}
