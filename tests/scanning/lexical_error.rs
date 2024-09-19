use crate::support::{trim_string, setup_command_environment};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn lexical_error() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, ",.$(#").unwrap();

    cmd.assert()
        .failure()
        .stdout(predicate::eq(trim_string(
            "
            COMMA , null
            DOT . null
            LEFT_PAREN ( null
            EOF  null
            ",
        )))
        .stderr(predicate::eq(trim_string(
            "
            [line 1] Error: Unexpected character: $
            [line 1] Error: Unexpected character: #
            ",
        )))
        .code(predicate::eq(65));
}
