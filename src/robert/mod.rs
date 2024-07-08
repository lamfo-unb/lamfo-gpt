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

    pub fn get_initial_system_msg(ctx: String) -> Message {
        let initial_message = Message {
            content: format!("
                
                Your name is Robert. You are an informational assistant about LAMFO (Machine Learning Laboratory in Finance and Organizations), and your job is to answer questions based on the past context of people who are interested in knowing more about the laboratory. Therefore, respond clearly and concisely.

                CONTEXT OBTAINED FROM DOCUMENTS OF LAMFO:
                \"\"\"
                {:?}
                \"\"\"

                If asked about something unrelated to LAMFO, you simply respond that you can only answer questions related to LAMFO.

                If you cannot find the user's answer in the context, you respond that you are unable to answer the user's question.
            ", ctx),
            role: message::TypeRole::System
        };

        initial_message
    }

    pub async fn send_message(&mut self, oac: &OaClient, messages: Vec<Message>) -> Result<()> {
        let new_message = Message::send_message(oac, messages).await?;
        self.messages.push(new_message);

        Ok(())
    }
}
