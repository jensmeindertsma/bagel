use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn parentheses() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(\"foo\")").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("(group foo)\n"));
}
