use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn success() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file("\"foo baz\"")
                .path()
                .to_str()
                .unwrap(),
        ],
    );

    output.success().stdout(predicate::str::diff(indoc! {"
        STRING \"foo baz\" foo baz
        EOF  null
    "}));
}

#[test]
fn failure() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file("\"bar").path().to_str().unwrap(),
        ],
    );

    output
        .code(predicate::eq(65))
        .stdout(predicate::str::diff(indoc! {"
            EOF  null
    "}))
        .stderr(predicate::str::diff(indoc! {"
            [line 1] Error: Unterminated string.
    "}));
}
