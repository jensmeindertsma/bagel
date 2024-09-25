use core::fmt::{self, Formatter};
use std::error::Error;

pub enum Command {
    Help,
    Parse { filename: String },
    Tokenize { filename: String },
}

pub trait TryFromIterator {
    type Item;
    type Error;

    fn try_from_iterator(iterator: impl Iterator<Item = Self::Item>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl TryFromIterator for Command {
    type Item = String;
    type Error = CommandParseError;

    fn try_from_iterator(
        mut iterator: impl Iterator<Item = Self::Item>,
    ) -> Result<Self, Self::Error> {
        let command = iterator.next().ok_or(CommandParseError::MissingCommand)?;

        match command.as_str() {
            "help" => Ok(Self::Help),
            "parse" => {
                let filename = iterator.next().ok_or(CommandParseError::MissingFilename)?;

                Ok(Self::Parse { filename })
            }
            "tokenize" => {
                let filename = iterator.next().ok_or(CommandParseError::MissingFilename)?;

                Ok(Self::Tokenize { filename })
            }
            _ => Err(CommandParseError::UnknownCommand(command)),
        }
    }
}

#[derive(Debug)]
pub enum CommandParseError {
    MissingCommand,
    MissingFilename,
    UnknownCommand(String),
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCommand => write!(f, "missing command"),
            Self::MissingFilename => write!(f, "missing filename"),
            Self::UnknownCommand(string) => write!(f, "unknown command `{string}`"),
        }
    }
}

impl Error for CommandParseError {}
