use std::sync::Arc;

use qdrant_client::Qdrant;
use tokio::sync::Mutex;

pub mod error;

use crate::embeddings::store::error::Result;

pub async fn new_qdrant_connect() -> Result<Arc<Mutex<Qdrant>>> {
    let client = Qdrant::from_url("http://qdrant:6334").build()?;

    Ok(Arc::new(Mutex::new(client)))
}