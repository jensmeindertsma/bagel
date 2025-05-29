use crate::common::{create_temporary_file, run_command};
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn test() {
    let output = run_command(
        crate::BINARY,
        &[
            "scan",
            create_temporary_file("foo bar _hello")
                .path()
                .to_str()
                .unwrap(),
        ],
    );

    output.success().stdout(predicate::eq(indoc! {"
        IDENTIFIER foo null
        IDENTIFIER bar null
        IDENTIFIER _hello null
        EOF  null
    "}));
}
