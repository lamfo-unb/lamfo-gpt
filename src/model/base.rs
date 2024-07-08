use super::Error;
use crate::{manager::AppManager, model::Result};
use sqlb::HasFields;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(app_manager: &AppManager, data: E) -> Result<i64>
where
    MC: DbBmc,
    E: HasFields,
{
    let db = app_manager.db();

    let fields = data.not_none_fields();
    let (id,) = sqlb::insert()
        .table(MC::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (i64,)>(db)
        .await
        .map_err(|err| Error::Sqlx(err.to_string()))?;

    Ok(id)
}
