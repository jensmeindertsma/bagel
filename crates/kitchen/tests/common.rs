use assert_cmd::Command;
use assert_fs::TempDir;
use std::{fs, path::PathBuf};

pub struct Test<'a> {
    pub binary: &'static str,
    pub arguments: &'a [&'a str],
    pub expected_status: Status,
    pub expected_stdout: Option<&'static str>,
    pub expected_stderr: Option<&'static str>,
}

impl Test<'_> {
    pub fn run(&self) {
        let mut command = Command::cargo_bin(self.binary).unwrap();

        command.args(self.arguments);

        command.env("COLORIZATION", "disabled");

        let output = command.output().unwrap();

        match self.expected_status {
            Status::Success => assert!(
                output.status.success(),
                "expected success with exit code `0`, found exit code `{}`",
                output.status
            ),
            // Status::Failure(exit_code) => {
            //     assert_eq!(
            //         exit_code,
            //         output.status.code().unwrap(),
            //         "expected failure with exit code {exit_code}, found exit code `{}`",
            //         output.status
            //     )
            // }
        }

        if let Some(expected) = self.expected_stdout {
            let output = String::from_utf8(output.stdout).unwrap();
            assert_eq!(
                expected, output,
                "output to `stdout` did not match expected value"
            );
        }

        if let Some(expected) = self.expected_stderr {
            let output = String::from_utf8(output.stderr).unwrap();
            assert_eq!(
                expected, output,
                "output to `stderr` did not match expected value"
            );
        }
    }
}

pub enum Status {
    Success,
    // Failure(i32),
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
