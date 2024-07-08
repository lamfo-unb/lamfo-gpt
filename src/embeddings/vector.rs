use std::sync::Arc;

use async_openai::types::Embedding;
use qdrant_client::{
    qdrant::{
        with_payload_selector::SelectorOptions, CreateCollectionBuilder, Distance, PointStruct,
        ScalarQuantizationBuilder, ScoredPoint, SearchPoints, UpsertPointsBuilder,
        VectorParamsBuilder, WithPayloadSelector,
    },
    Payload, Qdrant,
};
use serde_json::json;
use tokio::sync::Mutex;

use crate::embeddings::error::Result;

use super::file::File;

#[derive(Clone)]
pub struct VectorDB {
    client: Arc<Mutex<Qdrant>>,
    id: u64,
}

static COLLECTION: &str = "docs";

impl VectorDB {
    pub fn new(client: Arc<Mutex<Qdrant>>) -> Self {
        Self { client, id: 0 }
    }

    pub async fn reset_collection(&self) -> Result<()> {
        let client = self.client.lock().await;
    
        client.delete_collection(COLLECTION).await?;

        client
            .create_collection(
                CreateCollectionBuilder::new(COLLECTION).vectors_config(
                    VectorParamsBuilder::new(1536, Distance::Cosine)
                        .quantization_config(ScalarQuantizationBuilder::default()),
                ),
            )
            .await?;

        Ok(())
    }

    pub async fn upsert_embedding(&mut self, embedding: Embedding, file: &File) -> Result<()> {
        let payload: Payload = json!({
            "id": file.path.clone(),
        })
        .try_into()?;

        let vec: Vec<f32> = embedding.embedding;

        let points = vec![PointStruct::new(self.id, vec, payload)];

        let client = self.client.lock().await;

        client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION, points))
            .await?;
        self.id += 1;

        Ok(())
    }

    pub async fn search(&self, embedding: Embedding) -> Result<ScoredPoint> {
        let vec: Vec<f32> = embedding.embedding;

        let payload_selector = WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(true)),
        };

        let search_points = SearchPoints {
            collection_name: COLLECTION.to_string(),
            vector: vec,
            limit: 1,
            with_payload: Some(payload_selector),
            ..Default::default()
        };

        let client = self.client.lock().await;

        let search_result = client.search_points(search_points).await?;

        let result = search_result.result[0].clone();

        Ok(result)
    }
}
