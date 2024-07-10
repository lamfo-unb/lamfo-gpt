use crate::ais::message::Message as MessageAI;
use crate::{
    ais::message::TypeRole,
    embeddings::get_contents,
    manager::AppManager,
    model::message::{Message, MessageBmc, MessageForCreate},
    lamfo_gpt::LAMFOGPT,
    utils::message::format_msg_to_msg_ai,
    web::error::Result,
};
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;
use uuid::Uuid;

use crate::web::error::Error;

pub fn routes(app_manager: AppManager) -> Router {
    Router::new().route(
        "/api/robert/chat",
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

    let context = get_contents(app_manager.oac(), &content, &app_manager.embedding_state())
        .await
        .map_err(|err| Error::Embedding(err.to_string()))?;

    let mut messages: Vec<Message> =
        MessageBmc::get_by_session_id(&app_manager, session_id).await?;

    let content_with_template = LAMFOGPT::get_prompt_template(content, context);

    let mut message_c = MessageForCreate {
        content: content_with_template.content,
        typed_role: TypeRole::to_string(&content_with_template.role),
        session_id,
    };

    MessageBmc::create(&app_manager, message_c.clone()).await?;
    messages.push(Message::from(message_c));

    let messages_formatted_ai: Vec<MessageAI> =
        format_msg_to_msg_ai(messages).map_err(|err| Error::Utils(err.to_string()))?;
    let response_message =
        MessageAI::send_message(app_manager.oac(), messages_formatted_ai).await?;

    message_c = MessageForCreate {
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
