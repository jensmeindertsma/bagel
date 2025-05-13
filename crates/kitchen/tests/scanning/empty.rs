use crate::common::{create_temporary_file, run_command};
use indoc::indoc;

#[test]
fn empty() {
    let contents = "";

    let output = run_command(
        crate::BINARY,
        &[
            "tokenize",
            create_temporary_file(contents).path().to_str().unwrap(),
        ],
    );

    output.success().stdout(indoc! {"
        EOF  null
    "});
}
