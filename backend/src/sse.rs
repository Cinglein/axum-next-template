use crate::{error::ServerErr, message::Message};
use axum::response::sse::{Event, Sse};
use chrono::Utc;
use futures_util::{stream::repeat_with, Stream};
use tokio::time::Duration;
use tokio_stream::StreamExt;

pub const SSE_HANDLER_PATH: &str = "/sse";

#[utoipa::path(
    get,
    path = SSE_HANDLER_PATH,
    params(),
    responses(
        (status = 200, description = "A SSE stream of Message values", content_type = "text/event-stream", body = Message),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, ServerErr>>> {
    let stream = repeat_with(|| {
        Event::default()
            .json_data(Message {
                data: format!("Hello, world from a SSE! {:?}", Utc::now()),
            })
            .map_err(ServerErr::MessageStreamErr)
    })
    .throttle(Duration::from_secs(10));
    Sse::new(stream).keep_alive(Default::default())
}
