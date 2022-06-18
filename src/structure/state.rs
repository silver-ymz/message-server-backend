use axum::extract::ws::WebSocket;

#[derive(Debug)]
pub struct WsState {
    pub socket: WebSocket,
    pub username: String,
}
