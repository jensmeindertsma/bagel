use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::{ffi::OsStr, fs::File};

#[test]
fn empty_file() {
    let (mut cmd, temp_dir) = setup_command_environment(["tokenize", "file.lox"]);

    File::create(temp_dir.join("file.lox")).unwrap();

    cmd.assert().success().stdout(predicate::eq("EOF  null\n"));
}

fn setup_command_environment<Item>(arguments: impl IntoIterator<Item = Item>) -> (Command, TempDir)
where
    Item: AsRef<OsStr>,
{
    let mut cmd = Command::cargo_bin("bagel").unwrap();
    for argument in arguments {
        cmd.arg(argument);
    }

    let temporary_directory = TempDir::new().unwrap();

    cmd.current_dir(&temporary_directory);

    (cmd, temporary_directory)
}
