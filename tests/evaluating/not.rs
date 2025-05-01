use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn boolean() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "!true").unwrap();

    cmd.assert().success().stdout(predicate::eq("false\n"));
}

#[test]
fn number() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "!10.40").unwrap();

    cmd.assert().success().stdout(predicate::eq("false\n"));
}

#[test]
fn double_wrapped_boolean() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "!((false))").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}
