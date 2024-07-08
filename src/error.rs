use crate::model::store as DbStore;
use crate::{ais, embeddings, model, utils};
use crate::embeddings::store as QdrantStore;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    Ais(ais::error::Error),
    UtilsError(utils::error::Error),
    Model(model::Error),
    Embedding(embeddings::error::Error),
    Qdrant(QdrantStore::error::Error),
    Postgres(DbStore::Error)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate