use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn comparison() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "83 < 99 < 115").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("(< (< 83.0 99.0) 115.0)\n"));
}
