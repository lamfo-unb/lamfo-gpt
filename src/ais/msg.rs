use async_openai::types::{CreateMessageRequest, MessageContent, MessageObject};
use super::error::{Result, Error};

pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
    CreateMessageRequest {
        role: "user".to_string(),
        content: content.into(),
        ..Default::default()
    }
}

pub fn asst_msg(content: impl Into<String>) -> CreateMessageRequest {
    CreateMessageRequest {
        role: "assistant".to_string(),
        content: content.into(),
        ..Default::default()
    }
}

pub fn get_text_content(msg: MessageObject) -> Result<String> {
    let msg_content = msg
        .content
        .into_iter()
        .next()
        .ok_or_else(|| "No message content found".to_string())?;

    let txt = match msg_content {
        MessageContent::Text(text) => text.text.value,
        MessageContent::ImageFile(_) => {
            return Err(Error::MessageImageNotSupportYet)
        }
    };

    Ok(txt)
}