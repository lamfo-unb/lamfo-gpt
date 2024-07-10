use axum::{middleware, Router};
use embeddings::embed_documentation;
use manager::AppManager;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use web::mw_session::mw_session;

use crate::error::Result;

mod _dev_utils;
mod ais;
mod config;
mod embeddings;
mod error;
mod model;
mod lamfo_gpt;
mod utils;
mod web;
mod manager;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .try_init()
        .expect("Erro to initialize tracing");

    _dev_utils::init_dev().await;

    let app_manager = AppManager::new().await?;

    // Embedding
    embed_documentation(
        app_manager.oac(),
        &mut app_manager.embedding_state().vector_db.clone(),
        &app_manager.embedding_state().files,
    )
    .await?;
    info!("Embedding Up");

    // let robert_controller = RobertController::new(robert, conv).await?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let routes_all = Router::new()
        .merge(web::routes_chat::routes(app_manager))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(session_layer)
                .layer(middleware::from_fn(mw_session)),
        );

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening {}", addr);
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}
