use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn one() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(18 * 3 / (3 * 6))").unwrap();

    cmd.assert().success().stdout(predicate::eq("3\n"));
}

#[test]
fn two() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "42 / 5").unwrap();

    cmd.assert().success().stdout(predicate::eq("8.4\n"));
}

#[test]
fn three() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(10.40 * 2) / 2").unwrap();

    cmd.assert().success().stdout(predicate::eq("10.4\n"));
}

#[test]
fn four() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "20 + 74 - (-(14 - 33))").unwrap();

    cmd.assert().success().stdout(predicate::eq("75\n"));
}

#[test]
fn five() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "70 - 65").unwrap();

    cmd.assert().success().stdout(predicate::eq("5\n"));
}

#[test]
fn six() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "69 - 93").unwrap();

    cmd.assert().success().stdout(predicate::eq("-24\n"));
}

#[test]
fn seven() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "10.40 - 2").unwrap();

    cmd.assert().success().stdout(predicate::eq("8.4\n"));
}

#[test]
fn eight() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "23 + 28 - (-(61 - 99))").unwrap();

    cmd.assert().success().stdout(predicate::eq("13\n"));
}
