use core::fmt::{self, Formatter};
use std::error::Error;

pub enum Command {
    Evaluate { filename: String },
    Help,
    Parse { filename: String },
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
            "evaluate" => {
                let filename = arguments
                    .next()
                    .ok_or(CommandError::MissingArgument("filename"))?;

                Ok(Self::Evaluate { filename })
            }

            "help" => Ok(Self::Help),

            "parse" => {
                let filename = arguments
                    .next()
                    .ok_or(CommandError::MissingArgument("filename"))?;

                Ok(Self::Parse { filename })
            }

            "tokenize" => {
                let filename = arguments
                    .next()
                    .ok_or(CommandError::MissingArgument("filename"))?;

                Ok(Self::Tokenize { filename })
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
