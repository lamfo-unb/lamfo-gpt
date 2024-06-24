use derive_more::From;

use crate::{ais, utils};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    DataDirNotFound,
    UtilsError(utils::error::Error),
    AisError(ais::error::Error),
    ReadError(std::io::Error),
    ShouldNotDeleteError(String),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate