use async_openai::types::{CreateEmbeddingRequestArgs, Embedding};
use file::File;
use finder::Finder;
use tracing::info;
use vector::VectorDB;
use crate::ais::OaClient;

use self::error::{ Result, Error };

pub mod file;
pub mod error;
pub mod store;
pub mod vector;
mod finder;

#[derive(Clone)]
pub struct EmbeddingState {
    pub files: Vec<File>,
    pub vector_db: VectorDB
}

pub async fn embed_file(oac: &OaClient,file: &File) -> Result<Vec<Embedding>> {
    let sentence_as_str: Vec<&str> = file.sentences.iter().map(|s| s.as_str()).collect();
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input(sentence_as_str)
        .build()?;
    
    let response = oac.embeddings().create(request).await?;
    
    Ok(response.data)
}

pub async fn embed_sentence(oac: &OaClient, prompt: &str) -> Result<Vec<Embedding>> {
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input(prompt)
        .build()?;
    
    let response = oac.embeddings().create(request).await?;
    
    Ok(response.data)
}

pub async fn embed_documentation(oac: &OaClient, vector_db: &mut VectorDB, files: &Vec<File>) -> Result<()> {
    for file in files {
        let embeddings = embed_file(oac, file).await?;
        info!("Embedding: {:?}", file.path);
        for embedding in embeddings {
            vector_db.upsert_embedding(embedding, file).await?;
        }
    }

    Ok(())
}

pub async fn get_contents(oac: &OaClient, prompt: &str, app_state: &EmbeddingState) -> Result<String> {
    let embedding = embed_sentence(oac, prompt).await?;
    let result = app_state.vector_db.search(embedding[0].clone()).await?;
    let content = app_state
        .files
        .get_contents(&result)
        .ok_or(Error::PromptError)?;
    Ok(content)
}