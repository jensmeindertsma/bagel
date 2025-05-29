use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn slash() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file("/()").path().to_str().unwrap(),
        ],
    );

    output.success().stdout(predicate::str::diff(indoc! {"
        SLASH / null
        LEFT_PAREN ( null
        RIGHT_PAREN ) null
        EOF  null
    "}));
}

#[test]
fn comment() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file("() // Comment")
                .path()
                .to_str()
                .unwrap(),
        ],
    );

    output.success().stdout(predicate::str::diff(indoc! {"
        LEFT_PAREN ( null
        RIGHT_PAREN ) null
        EOF  null
    "}));
}
