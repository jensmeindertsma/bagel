mod setup;

use predicates::prelude::*;
use setup::setup_command_environment;
use std::fs::File;

#[test]
fn empty_file() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "test.lox"]);

    File::create(temp_dir.join("test.lox")).unwrap();

    cmd.assert().success().stdout(predicate::eq("EOF  null\n"));
}
