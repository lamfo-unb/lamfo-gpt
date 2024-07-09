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
                You are an assistant for question-answering tasks for LAMFO (Machine Learning Laboratory in Finance and Organizations). 
                Use the following pieces of retrieved context to answer the question. 
                If you don't know the answer, just say that you don't know. 
                If the question is not about LEMFO, just say that you do not answer this type of question.
                Use three sentences maximum and keep the answer concise.

                Question: {:?}

                Context: {:?}

                Answer:
            ", question, ctx),
            role: message::TypeRole::Assistant
        };

        initial_message
    }

    pub async fn send_message(&mut self, oac: &OaClient, messages: Vec<Message>) -> Result<()> {
        let new_message = Message::send_message(oac, messages).await?;
        self.messages.push(new_message);

        Ok(())
    }
}
