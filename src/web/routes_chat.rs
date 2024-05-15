use axum::{extract::State, http::{header::CONTENT_TYPE, Method}, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use crate::{model::RobertController, web::error::{Result, Error}};

pub fn routes(robert_controller: RobertController) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);
    Router::new()
        .route("/api/robert/chat", post(robert_chat)
        .with_state(robert_controller)
        .layer(cors)
    )
}

async fn robert_chat(
    State(rc): State<RobertController>,
    Json(message_payload): Json<MessagePayLoad>,
) -> Result<Json<MessagePayLoad>> {
    let robert = rc.robert;
    let conv = rc.conv;
    let message = message_payload.message;
    
    let res = robert.chat(&conv, &message).await.map_err(|_| Error::SendingMessageChatError)?;

    Ok(
        Json(
            MessagePayLoad {
                message: res
            }
        )
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct MessagePayLoad {
    pub message: String
}
