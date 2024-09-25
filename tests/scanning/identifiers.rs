use crate::support::{setup_command_environment, trim_string};
use predicates::prelude::*;
use std::{fs::File, io::Write};

#[test]
fn braces() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    let mut file = File::create(temp_dir.join("test.lox")).unwrap();

    write!(file, "foo bar _hello").unwrap();

    cmd.assert().success().stdout(predicate::eq(trim_string(
        "
        IDENTIFIER foo null
        IDENTIFIER bar null
        IDENTIFIER _hello null
        EOF  null
        ",
    )));
}
