use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn equal() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"baz\" == \"baz\"").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("(== baz baz)\n"));
}

#[test]
fn not_equal() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "\"foo\" != \"baz\"").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("(!= foo baz)\n"));
}
