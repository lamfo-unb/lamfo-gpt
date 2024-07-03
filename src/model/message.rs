use serde::Deserialize;
use sqlb::Fields;

use crate::model::Result;

use super::{base::{self, DbBmc}, ModelManager};

#[derive(Deserialize, Fields)]
pub struct MessageForCreate {
    pub content: String,
    pub typed_role: String,
    pub session_id: String,
}

pub struct MessageBmc;

impl DbBmc for MessageBmc {
    const TABLE: &'static str = "message";
}

impl MessageBmc {
    pub async fn create(mm: &ModelManager ,message_c: MessageForCreate) -> Result<i64> {
        base::create::<Self, _>(mm, message_c).await
    }
}