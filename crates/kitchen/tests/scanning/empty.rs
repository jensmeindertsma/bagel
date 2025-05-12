use crate::common::{Status, TemporaryFile, Test};
use indoc::indoc;

#[test]
fn empty() {
    let file = TemporaryFile::new("");

    Test {
        binary: crate::BINARY,
        arguments: &["tokenize", file.path().to_str().unwrap()],
        expected_status: Status::Success,
        expected_stdout: Some(indoc! {"
            EOF  null
        "}),
        expected_stderr: None,
    }
    .run();
}
