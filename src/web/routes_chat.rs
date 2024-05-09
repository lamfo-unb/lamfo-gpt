use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{model::RobertController, web::error::{Result, Error}};

pub fn routes(robert_controller: RobertController) -> Router {
    Router::new()
        .route("/robert/chat", get(robert_chat)
        .with_state(robert_controller)
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
