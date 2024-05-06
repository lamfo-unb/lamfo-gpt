use crate::{ais::asst::CreateConfig, error::{Error, Result}};

mod ais;
mod config;
mod error;

#[tokio::main]
async fn main() {
    println!("Iniciando o chat!");

    match start().await {
        Ok(_) => println!("\nTchau!\n"),
        Err(e) => println!("\nError: {}\n", e),
    }
    
}

async fn start() -> Result<()> {
    let oac = ais::new_oa_client()
        .map_err(|_| Error::FailedToCreateOaClient)?;

    let asst_config = CreateConfig {
        name: "Robert".to_string(),
        model: "gpt-3.5-turbo".to_string(),
    };
    let asst_id = ais::asst::load_or_create_asst(&oac, asst_config, false)
        .await
        .map_err(|err| Error::FailedToCreateAssistant(err))?;

    println!("->> asst_id: {asst_id}");

    Ok(())
}