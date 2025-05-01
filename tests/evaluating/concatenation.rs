use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn one() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"hello\" + \" world!\"").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("hello world!\n"));
}

#[test]
fn two() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"42\" + \"24\"").unwrap();

    cmd.assert().success().stdout(predicate::eq("4224\n"));
}

#[test]
fn three() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo\" + \"bar\"").unwrap();

    cmd.assert().success().stdout(predicate::eq("foobar\n"));
}
