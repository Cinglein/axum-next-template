use crate::message::Message;
use axum::{
    extract::ws::{Message as WsMsg, WebSocketUpgrade},
    response::IntoResponse,
};
use chrono::Utc;
use tokio::time::{sleep, Duration};

pub const WS_HANDLER_PATH: &str = "/ws";

#[utoipa::path(
    get,
    path = WS_HANDLER_PATH,
    params(),
    responses(
        (status = 101, description = "A WS stream of Message values", body = Message),
    )
)]
pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(async |mut socket| loop {
        let text = serde_json::to_string(&Message {
            data: format!("Hello, world from a WS! {:?}", Utc::now()),
        })
        .expect("error serializing ws msg to string")
        .into();
        if socket.send(WsMsg::Text(text)).await.is_err() {
            break;
        }
        sleep(Duration::from_secs(10)).await;
    })
}
