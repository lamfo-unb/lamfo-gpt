use axum::{extract::Request, middleware::Next, response::Response};
use tower_sessions::Session;
use tracing::debug;
use uuid::Uuid;
use crate::web::error::{Result, Error};

const SESSION_ID_KEY: &str = "session_id";

pub async fn mw_session(
    session: Session,
    req: Request,
    next: Next
) -> Result<Response> {
    let session_id: Option<String> = session.get("session_id").await.map_err(|err| Error::SessionError(err.to_string()))?;

    if session_id.is_none() {
        session.insert(SESSION_ID_KEY, Uuid::new_v4().to_string()).await.map_err(|err| Error::SessionError(err.to_string()))?;
        debug!("Session id criado!");
    }

    Ok(next.run(req).await)
}