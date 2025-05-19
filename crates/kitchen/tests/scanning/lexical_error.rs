use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn test() {
    let output = run_command(
        crate::BINARY,
        &[
            "tokenize",
            create_temporary_file(",.$(#").path().to_str().unwrap(),
        ],
    );

    output
        .code(predicate::eq(65))
        .stdout(predicate::eq(indoc! {"
            COMMA , null
            DOT . null
            LEFT_PAREN ( null
            EOF  null
    "}))
        .stderr(predicate::eq(indoc! {"
            [line 1] Error: Unexpected character: $
            [line 1] Error: Unexpected character: #
    "}));
}
