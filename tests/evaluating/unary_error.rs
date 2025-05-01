use crate::support::{multiline_output, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn one() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "-\"hello world!\"").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operand must be a number.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}

#[test]
fn two() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "-\"foo\"").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operand must be a number.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}

#[test]
fn three() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "-true").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operand must be a number.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}

#[test]
fn four() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "-(\"foo\" + \"bar\")").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operand must be a number.
            [line 1]
            ",
        )));
}

#[test]
fn five() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "-false").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::eq(multiline_output(
            "
            Operand must be a number.
            [line 1]
            ",
        )))
        .code(predicate::eq(70));
}
