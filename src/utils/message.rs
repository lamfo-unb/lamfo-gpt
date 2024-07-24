use std::str::FromStr;

use crate::model::message::Message;
use crate::ais::message::{Message as MessageAi, TypeRole};

use crate::utils::error::{ Result, Error };

pub fn format_msg_to_msg_ai(messages: Vec<Message>) -> Result<Vec<MessageAi>> {
    let mut messages_formatted_ai: Vec<MessageAi> = Vec::new();

    for message in messages {
        let role = TypeRole::from_str(&message.typed_role).map_err(|err| Error::NoRoleDefined(err.to_string()))?;
        let message_formatted_ai = MessageAi {
            role,
            content: message.content
        };
        messages_formatted_ai.push(message_formatted_ai);
    }

    Ok(messages_formatted_ai)
}