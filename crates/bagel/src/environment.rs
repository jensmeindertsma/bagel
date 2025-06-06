use std::env;

pub enum Environment {
    Regular,
    Testing,
}

impl Environment {
    pub fn determine() -> Self {
        match env::var("CODECRAFTERS") {
            Ok(value) if value == "yes" => Self::Testing,
            _ => Self::Regular,
        }
    }
}
