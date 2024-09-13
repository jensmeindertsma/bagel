mod setup;

use predicates::prelude::*;
use setup::setup_command_environment;
use std::{fs::File, io::Write};

#[test]
fn empty_file() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    File::create(temp_dir.join("test.lox")).unwrap();

    cmd.assert().success().stdout(predicate::eq("EOF  null\n"));
}

#[test]
fn parentheses() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(()").unwrap();

    cmd.assert().success().stdout(predicate::eq(
        "LEFT_PAREN ( null
        LEFT_PAREN ( null
        RIGHT_PAREN ) null
        EOF  null\n",
    ));
}
