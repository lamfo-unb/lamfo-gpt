pub type Result<T> = core::result::Result<T, Error>;
use derive_more::From;
use serde::Serialize;

use crate::{embeddings, utils};

#[derive(Debug, From, Serialize, Clone)]
pub enum Error {
    OpenAIError(String),
    #[from(ignore)]
    SerdeJsonError(String),
    NoRoleDefined,
    #[from(ignore)]
    FunctionCallError(String),
    #[from(ignore)]
    EmbeddingError(String),
    NoFunctionDefined,
    Utils(utils::error::Error)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate