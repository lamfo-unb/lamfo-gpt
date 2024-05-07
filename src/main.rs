use textwrap::wrap;

use crate::{error::Result, robert::Robert, utils::cli::{ico_check, ico_res, prompt, txt_res}};

mod ais;
mod config;
mod error;
mod robert;
mod utils;

const DEFAULT_DIR: &str = "robert";

#[derive(Debug)]
enum Cmd {
    Quit,
    Chat(String),
    RefreshAll,
    RefreshConv,
    RefreshInst,
    RefreshFiles,
}

impl Cmd {
    fn from_input(input: impl Into<String>) -> Self {
        let input = input.into();

        if input == "/q" {
            Self::Quit
        } else if input == "/r" || input == "/ra" {
            Self::RefreshAll
        } else if input == "/ri" {
            Self::RefreshInst
        } else if input == "/rf" {
            Self::RefreshFiles
        } else if input == "/rc" {
            Self::RefreshConv
        } else {
            Self::Chat(input.to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("{} Initializing chat!", ico_check());

    match start().await {
        Ok(_) => println!("\nTchau!\n"),
        Err(e) => println!("\nError: {}\n", e),
    }
    
}

async fn start() -> Result<()> {
    let mut robert = Robert::init_from_dir(DEFAULT_DIR, false).await?;

    let mut conv = robert.load_or_create_conv(false).await?;

    loop {
        println!();
        let input = prompt("Ask away")?;
        let cmd = Cmd::from_input(input);

        match cmd {
            Cmd::Quit => break,
            Cmd::Chat(msg) => {
                let res = robert.chat(&conv, &msg).await?;
                let res = wrap(&res, 80).join("\n");
                println!("{} {}", ico_res(), txt_res(res));
            },
            Cmd::RefreshAll => {
                robert = Robert::init_from_dir(DEFAULT_DIR, true).await?;
                conv = robert.load_or_create_conv(true).await?;
            },
            Cmd::RefreshConv => {
                conv = robert.load_or_create_conv(true).await?;
            },
            Cmd::RefreshInst => {
                robert = Robert::init_from_dir(DEFAULT_DIR, true).await?;
                conv = robert.load_or_create_conv(true).await?;
            },
            Cmd::RefreshFiles => {
                robert.upload_files(true).await?;
                conv = robert.load_or_create_conv(true).await?;
            },
            
        }
    }

    Ok(())
}