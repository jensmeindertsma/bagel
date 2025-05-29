use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn test() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file(indoc! {"
                # (
                )\t@
            "})
            .path()
            .to_str()
            .unwrap(),
        ],
    );

    output
        .code(predicate::eq(65))
        .stdout(predicate::str::diff(indoc! {"
            LEFT_PAREN ( null
            RIGHT_PAREN ) null
            EOF  null
    "}))
        .stderr(predicate::str::diff(indoc! {"
            [line 1] Error: Unexpected character: #
            [line 2] Error: Unexpected character: @
        "}));
}
