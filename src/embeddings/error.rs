use async_openai::error::OpenAIError;
use derive_more::From;
use qdrant_client::QdrantError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    OpenAIError(OpenAIError),
    QdrantError(QdrantError),
    PromptError,
    Tokio(std::io::Error)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate