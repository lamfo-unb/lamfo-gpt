use tokio::sync::OnceCell;
use tracing::info;

use crate::manager::AppManager;

mod dev_db;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

pub async fn init_test() -> AppManager {
    static INIT: OnceCell<AppManager> = OnceCell::const_new();

    let app_manager = INIT.get_or_init(|| async {
        init_dev().await;
        AppManager::new().await.unwrap()
    })
    .await;

    app_manager.clone()
}