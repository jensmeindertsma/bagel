use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn integer() {
    let output = run_command(
        crate::BINARY,
        &[
            "tokenize",
            create_temporary_file("42").path().to_str().unwrap(),
        ],
    );

    output.success().stdout(predicate::eq(indoc! {"
            NUMBER 42 42.0
            EOF  null
    "}));
}

#[test]
fn decimal() {
    let output = run_command(
        crate::BINARY,
        &[
            "tokenize",
            create_temporary_file("1234.1234").path().to_str().unwrap(),
        ],
    );

    output.success().stdout(predicate::eq(indoc! {"
        NUMBER 1234.1234 1234.1234
        EOF  null
    "}));
}
