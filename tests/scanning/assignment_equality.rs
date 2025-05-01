use crate::support::{multiline_output, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn assignment_equality_operators() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "={===}";

    write!(file, "{contents}").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(multiline_output(
            "
        EQUAL = null
        LEFT_BRACE { null
        EQUAL_EQUAL == null
        EQUAL = null
        RIGHT_BRACE } null
        EOF  null
        ",
        )));
}
