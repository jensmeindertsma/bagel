use crate::support::{setup_command_environment, multiline_output};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn braces() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "{{}}";

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(multiline_output(
        "
        LEFT_BRACE { null
        LEFT_BRACE { null
        RIGHT_BRACE } null
        RIGHT_BRACE } null
        EOF  null
        ",
    )));
}
