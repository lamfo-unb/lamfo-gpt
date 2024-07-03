use store::{new_db_pool, Db};

mod error;
mod store;
mod base;
pub mod message;

use crate::ais::{new_oa_client, OaClient};
pub use crate::model::error::{ Result, Error};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
    oac: OaClient,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        let oac = new_oa_client()?;

        Ok(ModelManager { db, oac })
    }

    pub (in crate::model) fn db(&self) -> &Db {
        &self.db
    }

    pub fn oac(&self) -> &OaClient {
        &self.oac
    }
}