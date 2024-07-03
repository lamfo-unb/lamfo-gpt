use crate::{ais::{message::{self, Message}, OaClient}, config::config};

mod error;

use crate::robert::error::Result;

pub struct RobertAI {
    pub model: String,
    pub messages: Vec<Message>
}

impl RobertAI {
    pub fn new(messages: Vec<Message>) -> RobertAI {
        RobertAI {
            model: config().model_chat_oa.clone(),
            messages: messages
        }
    }

    pub fn get_initial_system_msg() -> Message {
        let initial_message = Message {
            content: format!("
            Your name is Robert and is specialist information of LAMFO (Machine Learning Laboratory in Finance and Organizations).

            If you area asked about anything to do not with LAMFO,
            Answer that I answer omly questions about LAMFO.

            If you are asked about LAMFO,
            Answer that LAMFO is a best laboratory.
            "),
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
