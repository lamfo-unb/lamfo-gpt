use std::str::FromStr;

use async_openai::types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, Role};

use crate::{ais::{ Error, Result }, config::config};

use super::OaClient;

#[derive(Clone)]
pub struct Message {
    pub role: TypeRole,
    pub content: String
}

#[derive(Clone)]
pub enum TypeRole {
    Assistant,
    User,
    System
}

impl FromStr for TypeRole {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "user" => Ok(TypeRole::User),
            "assistant" => Ok(TypeRole::Assistant),
            "system" => Ok(TypeRole::System),
            _ => Err(())
        }
    }
}

impl Message {
    pub async fn send_message(oac: &OaClient, messages: Vec<Message>) -> Result<Message> {
        let mut messages_formatted = Vec::new();
        let messages = messages.clone().into_iter();
        let chat_model = &config().model_chat_oa;

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
                    let message_formatted = ChatCompletionRequestAssistantMessageArgs::default()
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
                _ => return Err(Error::NoRoleDefined)
            };
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(chat_model)
            .messages(messages_formatted)
            .build()
            .map_err(|err| Error::OpenAIError(err.to_string()))?;

        let response = oac.chat().create(request).await.map_err(|err| Error::OpenAIError(err.to_string()))?;

        let new_msg_role = response.choices[0].message.role;
        let response_role: TypeRole;

        match new_msg_role {
            Role::System => response_role = TypeRole::System,
            Role::Assistant => response_role = TypeRole::Assistant,
            Role::User => response_role = TypeRole::User,
            _ => return Err(Error::NoRoleDefined)
        }

        let response_msg = Message {
            role: response_role,
            content: response.choices[0]
                .message
                .content
                .clone()
                .unwrap_or("".to_string()),
        };

        Ok(response_msg)
    }
}