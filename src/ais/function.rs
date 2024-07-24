use crate::{
    ais::error::{Error, Result},
    config::config,
    embeddings::{get_contents, EmbeddingState},
    lamfo_gpt::{self, LAMFOGPT},
};
use async_openai::types::{
    ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, ChatCompletionTool, ChatCompletionToolArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs, CreateChatCompletionResponse, FunctionObject
};
use serde_json::{json, Value};
use tracing::debug;

use super::OaClient;

pub async fn call_function_tool(
    tool_calls: Vec<ChatCompletionMessageToolCall>,
    oac: &OaClient,
    embedding_state: &EmbeddingState,
) -> Result<CreateChatCompletionResponse> {
    let chat_model = &config().model_chat_oa;

    for tool_call in tool_calls.iter() {
        let tool_call_id = tool_call.id.clone();
        let fn_name = tool_call.function.name.clone();
        let params: Value = serde_json::from_str(&tool_call.function.arguments)
            .map_err(|err| Error::SerdeJsonError(err.to_string()))?;

        match fn_name.as_str() {
            "get_contents" => {
                debug!("Calling get_contents function");
                let prompt = params["question"].as_str().unwrap();
                debug!("Prompt: {}", prompt);
                let completion = get_contents(&oac, &embedding_state, prompt)
                    .await
                    .map_err(|err| Error::EmbeddingError(err.to_string()))?;
                let prompt_with_context = LAMFOGPT::get_prompt_template_with_context(&completion);
                let message_formatted: ChatCompletionRequestMessage =
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(prompt_with_context.content)
                        .build()
                        .map_err(|err| Error::OpenAIError(err.to_string()))?
                        .into();
                let request = CreateChatCompletionRequestArgs::default()
                    .model(chat_model)
                    .messages(vec![message_formatted])
                    .build()
                    .map_err(|err| Error::OpenAIError(err.to_string()))?;

                let response = oac
                    .chat()
                    .create(request)
                    .await
                    .map_err(|err| Error::OpenAIError(err.to_string()))?;

                return Ok(response)
            }
            _ => return Err(Error::NoFunctionDefined),
        }
    }
    
    Err(Error::NoFunctionDefined)
}

pub fn get_functions_schemas() -> Result<Vec<ChatCompletionTool>> {
    let mut functions_call = Vec::new();

    let get_contents_function_schema = ChatCompletionToolArgs::default()
        .function(FunctionObject {
            name: "get_contents".into(),
            description: Some("Get the informations about LAMFO of a given question".into()),
            parameters: Some(json!({
                    "type": "object",
                    "properties": {
                        "question": {
                            "type": "string",
                            "description": "User asked question about LAMFO"
                        }
                    },
                    "required": ["question"]
            }
        )),
    })
    .build()
    .map_err(|err| Error::OpenAIError(err.to_string()))?;;

    functions_call.push(get_contents_function_schema);


    Ok(functions_call)
}