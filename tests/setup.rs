use assert_cmd::Command;
use assert_fs::TempDir;
use std::ffi::OsStr;

pub fn setup_command_environment<Item>(
    arguments: impl IntoIterator<Item = Item>,
) -> (Command, TempDir)
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
