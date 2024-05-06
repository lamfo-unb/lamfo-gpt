use crate::{ais::asst::{self, CreateConfig}, error::{Error, Result}};

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
    asst::upload_instructions(
        &oac, 
        &asst_id, 
        r#"
        Your name is Robert and is specialist information of LAMFO (Machine Learning Laboratory in Finance and Organizations).

        Your language is Portuguese.

        If you area asked about anything to do with LAMFO,
        Answer that I answer omly questions about LAMFO.

        Please review the knowledge bundle document before answering, and answer to the best of yout ability.

        Also, when user ask about LAMFO, check the knowledge file, everything is there.
        All the information about LAMFO is in this files.
        "#.to_string()
    ).await?;

    let thread_id = asst::create_thread(&oac).await?;
    let msg = asst::run_thread_msg(&oac, &asst_id, &thread_id, "Qual seu nome?").await?;
    println!("->> response: {msg}");

    Ok(())
}