use crate::{error::ServerErr, message::Message};
use axum::{extract::State, response::IntoResponse, Json};
use sqlx::{query, Row, SqlitePool};

pub const DB_HANDLER_PATH: &str = "/db";

#[utoipa::path(
    get,
    path = DB_HANDLER_PATH,
    params(),
    responses(
        (status = 200, description = "A Message retrieved from the DB", body = [Message]),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn db_handler(State(pool): State<SqlitePool>) -> Result<impl IntoResponse, ServerErr> {
    let data: String = query("SELECT 'Hello, world from the Database!';")
        .fetch_one(&pool)
        .await?
        .try_get(0)?;
    let res = Message { data };
    Ok(Json(res))
}
