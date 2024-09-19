use crate::support::{format_expected_output, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn relational_operators() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "<<=>>=";

    write!(file, "{contents}").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq(format_expected_output(
            "
            LESS < null
            LESS_EQUAL <= null
            GREATER > null
            GREATER_EQUAL >= null
            EOF  null
            ",
        )));
}
