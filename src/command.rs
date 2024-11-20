use core::fmt::{self, Formatter};
use std::error::Error;

pub enum Command {
    Help,
    Parse { filename: String },
    Play { input: String },
    Tokenize { filename: String },
}

impl Command {
    pub fn from_arguments(
        mut arguments: impl Iterator<Item = String>,
    ) -> Result<Self, CommandError> {
        match arguments
            .next()
            .ok_or(CommandError::MissingArgument("command"))?
            .as_str()
        {
            "help" => Ok(Self::Help),

            "tokenize" => {
                let filename = arguments
                    .next()
                    .ok_or(CommandError::MissingArgument("filename"))?;

                Ok(Self::Tokenize { filename })
            }

            "parse" => {
                let filename = arguments
                    .next()
                    .ok_or(CommandError::MissingArgument("filename"))?;

                Ok(Self::Parse { filename })
            }

            "play" => {
                let input = arguments
                    .next()
                    .ok_or(CommandError::MissingArgument("input"))?;

                Ok(Self::Play { input })
            }

            other => Err(CommandError::UnknownCommand(other.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    MissingArgument(&'static str),
    UnknownCommand(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArgument(argument) => write!(f, "missing argument `{argument}`"),
            Self::UnknownCommand(command) => write!(f, "unknown command `{command}`"),
        }
    }
}

impl Error for CommandError {}
