use std::env;

pub fn is_codecrafters_environment() -> bool {
    matches!(env::var("CODECRAFTERS"), Ok(value) if value == "yes")
}
