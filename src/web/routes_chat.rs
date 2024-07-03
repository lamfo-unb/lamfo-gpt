use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;
use crate::{model::{message::{MessageBmc, MessageForCreate}, ModelManager}, web::error::Result};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/robert/chat", post(robert_chat)
        .with_state(mm)
    )
}

async fn robert_chat(
    session: Session,
    State(mm): State<ModelManager>,
    Json(message_payload): Json<MessagePayLoad>,
) -> Result<Json<Value>> {
    let content = message_payload.content;
    let session_id: String = session.get("session_id").await.unwrap().unwrap();
    
    let message_c = MessageForCreate {
        content,
        session_id,
        typed_role: "user".to_string(),
    };

    let id = MessageBmc::create(&mm, message_c).await?;

    Ok(
        Json(
            json!({
                "message": id
            })
        )
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct MessagePayLoad {
    pub content: String
}
