use serde::Deserialize;
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;
use crate::{manager::AppManager, model::error::Error};

use crate::model::Result;

use super::base::{self, DbBmc};

#[derive(Deserialize, Fields, Clone)]
pub struct MessageForCreate {
    pub content: String,
    pub typed_role: String,
    pub session_id: Uuid,
}

#[derive(Fields, FromRow)]
pub struct Message {
    pub content: String,
    pub typed_role: String
}

impl From<MessageForCreate> for Message {
    fn from(value: MessageForCreate) -> Self {
        Message {
            content: value.content,
            typed_role: value.typed_role
        }
    }
}

pub struct MessageBmc;

impl DbBmc for MessageBmc {
    const TABLE: &'static str = "message";
}

pub trait MessagesBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {
    
}

impl MessagesBy for Message{}

impl MessageBmc {
    pub async fn create(app_manager: &AppManager ,message_c: MessageForCreate) -> Result<i64> {
        base::create::<Self, _>(app_manager, message_c).await
    }

    pub async fn get_by_session_id<E>(app_manager: &AppManager, session_id: Uuid) -> Result<Vec<E>> where E: MessagesBy {
        let db = app_manager.db();

        let messages = sqlb::select()
            .table(Self::TABLE)
            .and_where("session_id", "=", session_id)
            .order_by("id")
            .fetch_all::<_, E>(db)
            .await
            .map_err(|err| Error::Sqlx(err.to_string()))?;
    
        Ok(messages)
    }
}