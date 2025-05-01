use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn fractional() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "10.40").unwrap();

    cmd.assert().success().stdout(predicate::eq("10.4\n"));
}

#[test]
fn whole() {
    let (mut cmd, temp_dir) = setup_command_environment(["evaluate", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "10").unwrap();

    cmd.assert().success().stdout(predicate::eq("10\n"));
}
