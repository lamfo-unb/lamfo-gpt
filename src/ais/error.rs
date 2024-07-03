pub type Result<T> = core::result::Result<T, Error>;
use async_openai::{error::OpenAIError, types::RunStatus};
use derive_more::From;
use serde::Serialize;

#[derive(Debug, From, Serialize, Clone)]
pub enum Error {
    OpenAIError(String),
    NoRoleDefined,
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate