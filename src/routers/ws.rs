use crate::web_socket::WEBSOCKET_SENDER;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    WEBSOCKET_SENDER.send(socket).await.unwrap();
}
