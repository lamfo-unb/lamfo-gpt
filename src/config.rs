use std::{env, sync::OnceLock};

use crate::error::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

pub struct Config {
    pub openai_api_key: String,
    pub db_url: String,
    pub model_chat_oa: String,
    pub qdrant_url: String,
    pub lamfo_gpt_dir: String,
    pub pg_dev_app_url: String,
    pub pg_dev_postgres_url: String
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(
            Config {
                openai_api_key: get_env("OPENAI_API_KEY")?,
                db_url: get_env("DB_URL")?,
                model_chat_oa: get_env("MODEL_CHAT_OA")?,
                qdrant_url: get_env("QDRANT_URL")?,
                lamfo_gpt_dir: get_env("LAMFO_GPT_DIR")?,
                pg_dev_app_url: get_env("PG_DEV_APP_URL")?,
                pg_dev_postgres_url: get_env("PG_DEV_POSTGRES_URL")?,
            }
        )
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}