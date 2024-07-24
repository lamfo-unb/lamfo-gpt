use async_openai::{config::OpenAIConfig, Client};
use crate::{ais::error::Result, config::config};
pub use crate::ais::error::Error;

pub mod error;
pub mod message;
pub mod function;

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    let api_key = &config().openai_api_key;
    let config = OpenAIConfig::new()
        .with_api_key(api_key);

    Ok(Client::with_config(config))
}