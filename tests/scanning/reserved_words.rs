use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn reserved_words() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    let contents = "and";

    write!(file, "{contents}").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
            AND and null
            EOF  null
            ",
    )));
}
