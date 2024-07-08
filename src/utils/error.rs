use std::path::{Path, StripPrefixError};

use derive_more::From;
use serde::Serialize;

use crate::ais;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FileErro(std::io::Error),
	Ais(ais::Error),
	StripPrefixFileError(StripPrefixError),
	NotAvaliableError,
}

impl From<StripPrefixError> for Error {
	fn from(val: StripPrefixError) -> Self {
		Self::StripPrefixFileError(val)
	}
}

impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Self::FileErro(val)
	}
}

impl From<ais::Error> for Error {
	fn from(val: ais::Error) -> Self {
		Self::Ais(val)
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