pub type Result<T> = core::result::Result<T, Error>;
use async_openai::{error::OpenAIError, types::RunStatus};
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    OpenAIError(OpenAIError),
    ConsoleWriteError(std::io::Error),
    WhileRunError(RunStatus),
    GetMessageError(std::string::String),
    MessageImageNotSupportYet,
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate