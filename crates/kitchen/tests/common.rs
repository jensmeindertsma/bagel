use assert_cmd::{Command, assert::Assert};
use assert_fs::TempDir;
use std::{fs, path::PathBuf};

pub fn run_command(binary: &str, arguments: &[&str]) -> Assert {
    let mut command = Command::cargo_bin(binary).unwrap();

    command.args(arguments);

    command.env("COLORIZATION", "disabled");

    command.assert()
}

pub struct TemporaryFile {
    path: PathBuf,
}

impl TemporaryFile {
    pub fn new(contents: &str) -> Self {
        let directory = TempDir::new().unwrap();
        let path = directory.join("test.lox");

        fs::write(&path, contents).unwrap();

        Self { path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
