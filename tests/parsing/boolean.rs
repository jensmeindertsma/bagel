use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn true_test() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "true").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        true
        ",
    )));
}

#[test]
fn false_test() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "false").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        false
        ",
    )));
}
