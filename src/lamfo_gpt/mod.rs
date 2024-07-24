use crate::ais::message::{self, Message};

mod error;

pub struct LAMFOGPT {}

impl LAMFOGPT {
    pub fn get_prompt_template() -> Message {
        let initial_message = Message {
            content: format!("
                You are friendly chatbot for question-answering tasks about LAMFO (Machine Learning Laboratory in Finance and Organizations). 
                You natural language is Portuguese Brazil.
                If you are asked for any information about LAMFO, call the get_contents function.
                If you know how to respond without needing context, something that is relevant to the user, respond politely, but encouraging the user to ask about LAMFO
            "),
            role: message::TypeRole::System
        };

        initial_message
    }

    pub fn get_prompt_template_with_context(context: &str) -> Message {
        let initial_message = Message {
            content: format!("
                You are a friendly assistant to formulate a response about LAMFO, given a context that I will provide between ```.
                Your natural language is Brazilian Portuguese.
                If even given the context you do not know the question, simply respond that you do not know and that you will save the question to know how to answer it later.

                Context: ```{}```
                ",
                context
            ),
            role: message::TypeRole::System
        };

        initial_message
    }
}
