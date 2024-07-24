use crate::ais::message::Message as MessageAI;
use crate::{
    ais::message::TypeRole,
    embeddings::get_contents,
    lamfo_gpt::LAMFOGPT,
    manager::AppManager,
    model::message::{Message, MessageBmc, MessageForCreate},
    utils::message::format_msg_to_msg_ai,
    web::error::Result,
};
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;
use tracing::debug;
use uuid::Uuid;

use crate::web::error::Error;

pub fn routes(app_manager: AppManager) -> Router {
    Router::new().route(
        "/api/lamfo-gpt/chat",
        post(robert_chat).with_state(app_manager),
    )
}

async fn robert_chat(
    session: Session,
    State(app_manager): State<AppManager>,
    Json(message_payload): Json<MessagePayLoad>,
) -> Result<Json<Value>> {
    let content = message_payload.content;
    let session_id_str: String = session.get("session_id").await.unwrap().unwrap();
    let session_id =
        Uuid::parse_str(&session_id_str).map_err(|err| Error::UuidError(err.to_string()))?;

    let mut messages: Vec<Message> =
        MessageBmc::get_by_session_id(&app_manager, session_id).await?;
    
    if messages.is_empty() {
        let content_with_template = LAMFOGPT::get_prompt_template();

        let message_c = MessageForCreate {
            content: content_with_template.content,
            typed_role: TypeRole::to_string(&content_with_template.role),
            session_id,
        };
    
        MessageBmc::create(&app_manager, message_c.clone()).await?;
        messages.push(Message::from(message_c)); 
    }

    let message_c = MessageForCreate {
        content: content.clone(),
        typed_role: TypeRole::User.to_string(),
        session_id,
    };
    MessageBmc::create(&app_manager, message_c.clone()).await?;
    messages.push(Message::from(message_c));

    let messages_formatted_ai: Vec<MessageAI> =
        format_msg_to_msg_ai(messages).map_err(|err| Error::Utils(err.to_string()))?;

    let response_message = MessageAI::send_message(
        app_manager.oac(),
        messages_formatted_ai,
        &app_manager.embedding_state(),
    )
    .await?;

    let message_c = MessageForCreate {
        content: response_message.content.clone(),
        typed_role: TypeRole::to_string(&response_message.role),
        session_id,
    };
    MessageBmc::create(&app_manager, message_c.clone()).await?;

    let body = Json(json!({
        "result": {
            "response": response_message
        }
    }));

    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
struct MessagePayLoad {
    pub content: String,
}
