use async_openai::types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs};

use crate::ais::message::{Message, TypeRole};
use crate::utils::error::{ Result, Error };

pub fn format_messages_for_open_ai(messages: Vec<Message>) -> Result<Vec<ChatCompletionRequestMessage>> {
    let mut messages_formatted = Vec::new();

    for message in messages {
        match message.role {
            TypeRole::Assistant => {
                let message_formatted = ChatCompletionRequestAssistantMessageArgs::default()
                    .content(message.content)
                    .build()
                    .map_err(|err| Error::OpenAIError(err.to_string()))?
                    .into();
                messages_formatted.push(message_formatted);
            }
            TypeRole::User => {
                let message_formatted = ChatCompletionRequestUserMessageArgs::default()
                    .content(message.content)
                    .build()
                    .map_err(|err| Error::OpenAIError(err.to_string()))?
                    .into();
                messages_formatted.push(message_formatted);
            }
            TypeRole::System => {
                let message_formatted = ChatCompletionRequestSystemMessageArgs::default()
                    .content(message.content)
                    .build()
                    .map_err(|err| Error::OpenAIError(err.to_string()))?
                    .into();
                messages_formatted.push(message_formatted);
            }
            _ => return Err(Error::NoRoleDefined(message.role.to_string())),
        };
    }

    Ok(messages_formatted)
}