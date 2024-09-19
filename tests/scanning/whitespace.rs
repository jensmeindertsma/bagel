use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn comment() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "() // Comment";

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
            LEFT_PAREN ( null
            RIGHT_PAREN ) null
            EOF  null
            ",
    )));
}

#[test]
fn slash() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = trim_string(
        "
        (   
         )
        ",
    );

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
            LEFT_PAREN ( null
            RIGHT_PAREN ) null
            EOF  null
            ",
    )));
}
