use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CreateFileErro(std::io::Error),
    FileNotFound(String),
    ReadFileToStringErro,
    ConvertStrFromTomlError,
    GlobError(globset::Error),
    IsNotFile(String),
    SerdeJsonError(serde_json::Error),
    PromptError(dialoguer::Error)
}

impl From<globset::Error> for Error {
	fn from(val: globset::Error) -> Self {
		Self::GlobError(val)
	}
}

impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Self::CreateFileErro(val)
	}
}

impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Self::SerdeJsonError(val)
	}
}

impl From<dialoguer::Error> for Error {
	fn from(val: dialoguer::Error) -> Self {
		Self::PromptError(val)
	}
}


// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate