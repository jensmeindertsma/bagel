use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::fs;

#[test]
fn one() {
    let (mut cmd, temp_dir) = setup_command_environment(["run", "test.lox"]);

    fs::write(
        temp_dir.join("test.lox"),
        include_str!("./multiple/one.lox"),
    )
    .unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello, World!\n42"));
}
