use std::str::FromStr;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;
use uuid::Uuid;
use crate::{ais::{self, message::{self, TypeRole}}, model::{message::{Message, MessageBmc, MessageForCreate}, ModelManager}, robert::RobertAI, utils::message::format_msg_to_msg_ai, web::error::Result};
use crate::ais::message::Message as MessageAI;

use crate::web::error::Error;

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
    let session_id_str: String = session.get("session_id").await.unwrap().unwrap();
    let session_id = Uuid::parse_str(&session_id_str).map_err(|err| Error::UuidError(err.to_string()))?;

    let mut messages: Vec<Message> = MessageBmc::get_by_session_id(&mm, session_id).await?;

    if messages.is_empty() {
        let initial_messages: MessageAI = RobertAI::get_initial_system_msg();
        let message_c = MessageForCreate {
            content: initial_messages.content,
            typed_role: TypeRole::to_string(&initial_messages.role),
            session_id
        };

        MessageBmc::create(&mm, message_c.clone()).await?;
        messages.push(Message::from(message_c))
    }
    
    let mut message_c = MessageForCreate {
        content,
        session_id,
        typed_role: "user".to_string(),
    };

    MessageBmc::create(&mm, message_c.clone()).await?;
    messages.push(Message::from(message_c));

    let messages_formatted_ai: Vec<MessageAI> = format_msg_to_msg_ai(messages).map_err(|err| Error::Utils(err.to_string()))?;
    let response_message = MessageAI::send_message(mm.oac(), messages_formatted_ai).await?;

    message_c = MessageForCreate {
        content: response_message.content.clone(),
        typed_role: TypeRole::to_string(&response_message.role),
        session_id
    };
    MessageBmc::create(&mm, message_c.clone()).await?;

    let body = Json(json!({
        "result": {
            "response": response_message
        }
    }));

    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
struct MessagePayLoad {
    pub content: String
}
