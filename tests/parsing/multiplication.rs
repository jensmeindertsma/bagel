use crate::support::setup_command_environment;
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn multiplication() {
    let (mut cmd, temp_dir) = setup_command_environment(["parse", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "38 * 58").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("(* 38.0 58.0)\n"));
}
