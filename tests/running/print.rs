use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn print() {
    let (mut cmd, temp_dir) = setup_command_environment(["run", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "print \"Hello, World!\";").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello, World!\n"));
}
