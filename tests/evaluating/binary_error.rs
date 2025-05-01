use crate::support::{multiline_output, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn one() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo\" / 42").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operands must be numbers.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}

#[test]
fn two() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo\" * 42").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operands must be numbers.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}

#[test]
fn three() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "true / 2").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operands must be numbers.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}

#[test]
fn four() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(\"foo\" * \"bar\")").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operands must be numbers.
            [line 1]
            ",
        )));
}

#[test]
fn five() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "false / true").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operands must be numbers.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}
