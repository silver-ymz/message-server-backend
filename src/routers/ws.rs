use crate::structure::{state::WsState, auth::Claims};
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use tokio::sync::mpsc::{Sender, self};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    claims: Claims,
    Extension(websocket_sender): Extension<mpsc::Sender<WsState>>,
) -> impl IntoResponse {
    let username = claims.name().clone();
    println!("`{}` connected", username);

    ws.on_upgrade(move |socket| handle_socket(socket, username, websocket_sender.clone()))
}

async fn handle_socket(socket: WebSocket, username: String, sender: Sender<WsState>) {
    sender.send(WsState{socket, username}).await.unwrap();
}
