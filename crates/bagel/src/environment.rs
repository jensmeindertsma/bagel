use std::env;

// In order to pass the tests defined by CodeCrafters when submitting
// this implementation to their platform, we have to respect their
// defined output format for errors. This format is different from
// the preferred error format that has colorization and a more detailed
// description. To detect whether we are running on their platform, we
// set the environment variable `CODECRAFTERS` to `yes` inside `.codecrafters/run.sh`
// which is executed by CodeCrafters when running their tests.

// Additionally, this environment variable is also set when running the
// integration tests defined in the `kitchen` crate. This is because the
// test cases inside `kitchen` reflect the CodeCrafters test cases as closely
// as possible. This has to do with submitting your code to CodeCrafters taking
// a really long time. Running tests locally is much faster. By having the local
// integration tests reflect the CodeCrafters test cases, we can quickly verify
// locally whether our implementation passes the CodeCrafters test cases.

// Lastly we support setting the environment variable `COLORIZATION` to `disabled`
// to disable ANSI escape sequences that add colors and bold/italic/underline font features.
// Some terminals do not support these sequences so we allow it to be disabled. It is also
// important to disable colors during testing, because the test cases expect plain output.
// As such, we do it automatically for the CodeCrafters environment.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Environment {
    Regular,
    Testing,
}

impl Environment {
    pub fn determine() -> (Self, Colorization) {
        match env::var("CODECRAFTERS") {
            // CodeCrafters doesn't handle colors correctly
            Ok(value) if value == "yes" => (Self::Testing, Colorization::Disabled),

            _ => match env::var("COLORIZATION") {
                // Only if explicitly disabled do we not print colors.
                Ok(value) if value == "disabled" => (Self::Regular, Colorization::Disabled),
                _ => (Self::Regular, Colorization::Enabled),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colorization {
    Enabled,
    Disabled,
}
