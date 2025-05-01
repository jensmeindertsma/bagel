use crate::support::{setup_command_environment, multiline_output};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn reserved_words() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "and").unwrap();

    cmd.assert().success().stdout(predicate::eq(multiline_output(
        "
        AND and null
        EOF  null
        ",
    )));
}
