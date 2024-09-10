pub enum Command {
    Help,
    Tokenize { filename: String },
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

pub trait TryFromIterator {
    type Item;
    type Error;

    fn try_from_iterator(iterator: impl Iterator<Item = Self::Item>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
