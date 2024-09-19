use crate::support::{trim_string, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn parentheses() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "(()").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(trim_string(
            "
            LEFT_PAREN ( null
            LEFT_PAREN ( null
            RIGHT_PAREN ) null
            EOF  null
            ",
        )));
}
