use serde::Serialize;

use crate::ais;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    Store(store::Error),
    Sqlx(String),
    Ais(ais::Error)
}

impl From<store::Error> for Error {
	fn from(val: store::Error) -> Self {
		Self::Store(val)
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