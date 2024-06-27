use axum::{http::Method, response::Html, routing::get, Router};
use robert::Conv;
use textwrap::wrap;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    error::Result, model::RobertController, robert::Robert, utils::cli::{ico_check, ico_res, prompt, txt_res}
};

mod ais;
mod config;
mod error;
mod robert;
mod utils;
mod web;
mod model;

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
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .try_init()
        .expect("Erro to initialize tracing");

    let (robert, conv) = start_robert().await?;

    let robert_controller = RobertController::new(robert, conv).await?;

    let routes_all = Router::new()
        .merge(web::routes_chat::routes(robert_controller))
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any));

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening {}", addr);
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

async fn start_robert() -> Result<(Robert, Conv)> {
    let robert = Robert::init_from_dir(DEFAULT_DIR, false).await?;

    let conv = robert.load_or_create_conv(false).await?;

    Ok((robert, conv))
}
