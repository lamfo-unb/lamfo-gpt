use std::path::StripPrefixError;

use derive_more::From;
use serde::Serialize;

use crate::ais;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    FileErro(String),
	StripPrefixFileError(String),
	NotAvaliableError,
	NoRoleDefined(String),
	OpenAIError(String)
}


// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate