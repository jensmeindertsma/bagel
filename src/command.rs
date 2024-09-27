use core::fmt::{self, Formatter};
use std::error::Error;

pub enum Command {
    Help,
    Parse { filename: String },
    Play,
    Tokenize { filename: String },
}

impl Command {
    pub fn try_from_iterator(
        mut iterator: impl Iterator<Item = String>,
    ) -> Result<Self, CommandError> {
        let command = iterator.next().ok_or(CommandError::MissingCommand)?;

        match command.as_str() {
            "help" => Ok(Self::Help),
            "parse" => {
                let filename = iterator.next().ok_or(CommandError::MissingFilename)?;

                Ok(Self::Parse { filename })
            }
            "tokenize" => {
                let filename = iterator.next().ok_or(CommandError::MissingFilename)?;

                Ok(Self::Tokenize { filename })
            }
            _ => Err(CommandError::UnknownCommand(command)),
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    MissingCommand,
    MissingFilename,
    UnknownCommand(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCommand => write!(f, "missing command"),
            Self::MissingFilename => write!(f, "missing filename"),
            Self::UnknownCommand(string) => write!(f, "unknown command `{string}`"),
        }
    }
}

impl Error for CommandError {}
