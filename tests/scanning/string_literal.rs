use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn string() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo baz\"").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        STRING \"foo baz\" foo baz
        EOF  null
        ",
    )));
}

#[test]
fn unterminated_string() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "\"bar";

    write!(file, "{contents}").unwrap();

    cmd.assert()
        .failure()
        .stdout(predicate::eq(trim_string(
            "
            EOF  null
            ",
        )))
        .stderr(predicate::eq(trim_string(
            "
            [line 1] Error: Unterminated string.
            ",
        )));
}
