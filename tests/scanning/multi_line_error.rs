use crate::support::{setup_command_environment, multiline_output};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn multi_line_error() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = multiline_output(
        "
        # (
        )   @
        ",
    );

    write!(file, "{contents}").unwrap();

    cmd.assert()
        .failure()
        .stdout(predicate::eq(multiline_output(
            "
            LEFT_PAREN ( null
            RIGHT_PAREN ) null
            EOF  null
            ",
        )))
        .stderr(predicate::eq(multiline_output(
            "
            [line 1] Error: Unexpected character: #
            [line 2] Error: Unexpected character: @
            ",
        )))
        .code(predicate::eq(65));
}
