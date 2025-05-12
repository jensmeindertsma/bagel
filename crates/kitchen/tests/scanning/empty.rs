use crate::common::{TemporaryFile, run_command};
use indoc::indoc;

#[test]
fn empty() {
    let file = TemporaryFile::new("");

    let output = run_command(crate::BINARY, &["tokenize", file.path().to_str().unwrap()]);

    output.success().stdout(indoc! {"
        EOF  null
    "});
}
