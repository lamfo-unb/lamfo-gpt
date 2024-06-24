use crate::{ais, robert, utils};
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    FailedToCreateAssistant(ais::error::Error),
    CreateRobertError(robert::error::Error),
    UtilsError(utils::error::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate