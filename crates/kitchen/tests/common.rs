use assert_cmd::{Command, assert::Assert};
use assert_fs::NamedTempFile;
use std::fs;

pub fn run_command(binary: &str, arguments: &[&str]) -> Assert {
    let mut command = Command::cargo_bin(binary).unwrap();

    command.args(arguments);

    command.env("COLORIZATION", "disabled");

    command.assert()
}

pub fn create_temporary_file(contents: &str) -> NamedTempFile {
    let file = NamedTempFile::new("test.lox").unwrap();

    fs::write(file.path(), contents).unwrap();

    file
}
