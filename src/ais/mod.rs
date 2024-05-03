use async_openai::{config::OpenAIConfig, Client};
use crate::{ais::error::Result, config::config};

mod error;

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    let api_key = &config().openai_api_key;
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id("lamfo");

    Ok(Client::with_config(config))
}