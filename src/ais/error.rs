pub type Result<T> = core::result::Result<T, Error>;
use async_openai::error::OpenAIError;

#[derive(Debug)]
pub enum Error {
    OpenAIError(OpenAIError),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<OpenAIError> for Error {
	fn from(val: OpenAIError) -> Self {
		Self::OpenAIError(val)
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate