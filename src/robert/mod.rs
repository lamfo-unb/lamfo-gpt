use crate::{
    ais::{
        message::{self, Message},
        OaClient,
    },
    config::config,
};

mod error;

use crate::robert::error::Result;

pub struct RobertAI {
    pub model: String,
    pub messages: Vec<Message>,
}

impl RobertAI {
    pub fn new(messages: Vec<Message>) -> RobertAI {
        RobertAI {
            model: config().model_chat_oa.clone(),
            messages: messages,
        }
    }

    pub fn get_prompt_template(question: String, ctx: String) -> Message {
        let initial_message = Message {
            content: format!("
                You are friendly chatbot for question-answering tasks about LAMFO (Machine Learning Laboratory in Finance and Organizations). 
                Use three sentences maximum and keep the answer concise.
                You natural language is Portuguese Brazil.
                Use the following pieces of retrieved context to answer the question.
                If you don't know the answer or question goes outside the LAMFO context, just say that you don't know. 

                Question: {:?}

                Context: {:?}

                Answer:
            ", question, ctx),
            role: message::TypeRole::User
        };

        initial_message
    }

    pub async fn send_message(&mut self, oac: &OaClient, messages: Vec<Message>) -> Result<()> {
        let new_message = Message::send_message(oac, messages).await?;
        self.messages.push(new_message);

        Ok(())
    }
}
