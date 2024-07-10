use crate::ais::new_oa_client;
use crate::embeddings::store::new_qdrant_connect;
use crate::embeddings::vector::VectorDB;
use crate::model::store::new_db_pool;
use crate::{ais::OaClient, embeddings::EmbeddingState, model::store::Db};
use crate::utils::files::load_files_from_dir;

use crate::error::Result;

#[derive(Clone)]
pub struct AppManager {
    db: Db,
    oac: OaClient,
    embedding_state: EmbeddingState,
}

impl AppManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        let oac = new_oa_client()?;
        let qdrant_client = new_qdrant_connect()
            .await?;

        let files = load_files_from_dir("./robert/files".into(), "txt", &".".into())?;
        let vector_db = VectorDB::new(qdrant_client);

        vector_db
            .reset_collection()
            .await?;

        let embedding_state = EmbeddingState { files, vector_db };

        Ok(AppManager {
            db,
            oac,
            embedding_state,
        })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn oac(&self) -> &OaClient {
        &self.oac
    }

    pub fn embedding_state(&self) -> &EmbeddingState {
        &self.embedding_state
    }
}
