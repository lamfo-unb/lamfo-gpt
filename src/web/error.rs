use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::From;
use serde::Serialize;
use tracing::debug;

use crate::{ais, model, utils};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, From)]
pub enum Error {
    SessionError(String),
    Model(model::Error),
    #[from(ignore)]
    UuidError(String),
    Ais(ais::Error),
    #[from(ignore)]
    Utils(String)
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("model::Error {self:?}");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
