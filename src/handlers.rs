use axum::{
    extract::{State, ws::{WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    Json,
};
use crate::models::LogEntry;
use crate::state::SharedState;

pub async fn handle_log(
    State(state): State<SharedState>,
    Json(payload): Json<LogEntry>,
) -> &'static str {
    let _ = state.tx.send(payload.clone());
    println!("Log received from: {}", payload.app_id);
    "Log entry received"
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    let mut rx = state.tx.subscribe();
    while let Ok(log) = rx.recv().await {
        if let Ok(msg) = serde_json::to_string(&log) {
            if socket.send(axum::extract::ws::Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    }
}