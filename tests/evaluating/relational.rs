use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn one() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "10 > 5").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn two() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "57 > -65").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn three() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "11 >= 11").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn four() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(54 - 67) >= -(114 / 57 + 11)").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn five() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "156 == (89 + 67)").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn six() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"hello\" == \"world\"").unwrap();

    cmd.assert().success().stdout(predicate::eq("false\n"));
}

#[test]
fn seven() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo\" != \"bar\"").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn eight() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo\" == \"foo\"").unwrap();

    cmd.assert().success().stdout(predicate::eq("true\n"));
}

#[test]
fn nine() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "61 == \"61\"").unwrap();

    cmd.assert().success().stdout(predicate::eq("false\n"));
}
