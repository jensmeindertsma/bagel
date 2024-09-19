use crate::support::{trim_string, setup_command_environment};
use predicates::prelude::*;
use std::fs::File;

#[test]
fn empty_file() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    File::create(temp_dir.join("test.lox")).unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(trim_string(
            "
            EOF  null
            ",
        )));
}
