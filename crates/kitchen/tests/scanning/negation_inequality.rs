use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn test() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file("!!===").path().to_str().unwrap(),
        ],
    );

    output.success().stdout(predicate::eq(indoc! {"
        BANG ! null
        BANG_EQUAL != null
        EQUAL_EQUAL == null
        EOF  null
    "}));
}
