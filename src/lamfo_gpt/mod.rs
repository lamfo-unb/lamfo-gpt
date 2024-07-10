use crate::ais::message::{self, Message};

mod error;

pub struct LAMFOGPT {}

impl LAMFOGPT {
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
}
