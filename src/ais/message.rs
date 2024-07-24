use std::{collections::HashMap, future::Future, pin::Pin, str::FromStr};

use async_openai::types::{
    ChatCompletionFunctionsArgs, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionToolArgs, CreateChatCompletionRequestArgs, FunctionObject, Role,
};
use serde::Serialize;
use serde_json::{json, Value};

use crate::embeddings::error::Result as EmbeddingResult;
use crate::utils::ais::format_messages_for_open_ai;
use crate::{
    ais::{Error, Result},
    config::config,
    embeddings::{get_contents, EmbeddingState},
};

use super::function::{call_function_tool, get_functions_schemas};
use super::OaClient;

#[derive(Clone, Serialize, Debug)]
pub struct Message {
    pub role: TypeRole,
    pub content: String,
}

#[derive(Clone, Serialize, Debug)]
pub enum TypeRole {
    Assistant,
    User,
    System,
}

impl FromStr for TypeRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "user" => Ok(TypeRole::User),
            "assistant" => Ok(TypeRole::Assistant),
            "system" => Ok(TypeRole::System),
            _ => Err(Error::NoRoleDefined),
        }
    }
}

impl ToString for TypeRole {
    fn to_string(&self) -> String {
        match self {
            TypeRole::Assistant => "assistant".to_string(),
            TypeRole::User => "user".to_string(),
            TypeRole::System => "system".to_string(),
        }
    }
}

impl Message {
    pub async fn send_message(
        oac: &OaClient,
        messages: Vec<Message>,
        embedding_state: &EmbeddingState,
    ) -> Result<Message> {
        let messages = messages.clone();
        let chat_model = &config().model_chat_oa;

        let mut messages_formatted =
            format_messages_for_open_ai(messages)?;

        let functions_call = get_functions_schemas()?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(chat_model)
            .messages(messages_formatted)
            .tools(functions_call)
            .tool_choice("auto")
            .build()
            .map_err(|err| Error::OpenAIError(err.to_string()))?;

        let mut response = oac
            .chat()
            .create(request)
            .await
            .map_err(|err| Error::OpenAIError(err.to_string()))?;

        let new_msg = response.choices.first().unwrap().message.clone();
        let mut new_msg_role = new_msg.role;
        let response_role: TypeRole;

        if let Some(tool_calls) = new_msg.tool_calls {
            response = call_function_tool(tool_calls, &oac, &embedding_state).await?;
            let new_msg_function = response.choices.first().unwrap().message.clone();
            new_msg_role = new_msg_function.role;
        };

        match new_msg_role {
            Role::System => response_role = TypeRole::System,
            Role::Assistant => response_role = TypeRole::Assistant,
            Role::User => response_role = TypeRole::User,
            _ => return Err(Error::NoRoleDefined),
        };

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
