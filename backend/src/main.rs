use axum::{extract::FromRef, routing::get, Router};
use sqlx::{migrate, migrate::MigrateDatabase, Sqlite, SqlitePool};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use db::*;
use error::ServerErr;
use sse::*;
use ws::*;

pub mod db;
pub mod error;
pub mod message;
pub mod sse;
pub mod ws;

#[derive(Clone, FromRef)]
struct AppState {
    pool: SqlitePool,
}

#[derive(OpenApi)]
#[openapi(paths(db_handler, ws_handler, sse_handler))]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), ServerErr> {
    let dir = "frontend/out";
    let static_service = ServeDir::new(dir)
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new(format!("{dir}/404.html")));

    Sqlite::create_database("sqlite::memory:").await?;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate!().run(&pool).await?;

    let state = AppState { pool };

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route(SSE_HANDLER_PATH, get(sse_handler))
        .route(DB_HANDLER_PATH, get(db_handler))
        .route(WS_HANDLER_PATH, get(ws_handler))
        .fallback_service(static_service)
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
