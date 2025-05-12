use std::env::{self, VarError};

use tracing::warn;

#[derive(Clone, Copy, Debug)]
pub enum Colorization {
    Enabled,
    Disabled,
}

impl Colorization {
    pub fn determine() -> Self {
        match env::var("COLORIZATION") {
            Ok(value) => {
                if value == "disabled" {
                    Self::Disabled
                } else {
                    warn!(
                        "Encountered unknown value `{value}` reading `$COLORIZATION`, set it `disabled` to affect the output"
                    );
                    Self::Enabled
                }
            }
            Err(error) => match error {
                VarError::NotPresent => Self::Enabled,
                VarError::NotUnicode(_) => {
                    warn!("`$COLORIZATION` value must be valid Unicode");
                    Self::Enabled
                }
            },
        }
    }
}
