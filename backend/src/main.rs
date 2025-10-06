use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Error as AxumError, Router,
};
use chrono::Utc;
use futures_util::{stream::repeat_with, Stream};
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate,
    migrate::{MigrateDatabase, MigrateError},
    Error as SqlxError, Sqlite, SqlitePool,
};
use thiserror::Error;
use tokio::time::Duration;
use tokio_stream::StreamExt;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use ts_rs::TS;

#[derive(Error, Debug)]
pub enum ServerErr {
    #[error("Error creating message stream")]
    MessageStreamErr(#[from] AxumError),
    #[error("Error setting up sql server")]
    SqlxErr(#[from] SqlxError),
    #[error("Error migrating sql")]
    SqlxMigrateErr(#[from] MigrateError),
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
struct Message {
    pub data: String,
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, ServerErr>>> {
    let stream = repeat_with(|| {
        Event::default()
            .json_data(Message {
                data: format!("Hello, world! {:?}", Utc::now()),
            })
            .map_err(ServerErr::MessageStreamErr)
    })
    .throttle(Duration::from_secs(1));
    Sse::new(stream).keep_alive(Default::default())
}

#[tokio::main]
async fn main() -> Result<(), ServerErr> {
    let dir = "frontend/out";
    let static_service =
        ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/404.html")));

    Sqlite::create_database("sqlite::memory:").await?;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate!().run(&pool).await?;

    let app = Router::new()
        .route("/hello", get(sse_handler))
        .fallback_service(static_service)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
