use axum::extract::ws::Message;
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::{broadcast, mpsc};

use crate::structure::state::WsState;

pub fn create_websocket_sender() -> mpsc::Sender<WsState> {
    let (send, mut recv): (mpsc::Sender<WsState>, mpsc::Receiver<WsState>) = mpsc::channel(10);

    tokio::spawn(async move {
        let (tx, _) = broadcast::channel::<String>(20);

        while let Some(socket_state) = recv.recv().await {
            let tx0 = tx.clone();
            let mut rx0 = tx0.subscribe();

            let (mut ws_send, mut ws_recv) = socket_state.socket.split();
            let username = socket_state.username.clone();

            let recv_task = tokio::spawn(async move {
                while let Some(Ok(msg)) = ws_recv.next().await {
                    match msg {
                        Message::Text(msg) => {
                            if let Err(e) = tx0.send(format!("{} {}", username.clone(), msg)) {
                                println!("{}", e);
                            }
                        }
                        Message::Close(_) => break,
                        _ => ()
                    }
                }
            });

            let send_task = tokio::spawn(async move {
                while let Ok(msg) = rx0.recv().await {
                    if let Err(e) = ws_send.send(Message::Text(msg)).await {
                        println!("{}", e);
                    }
                }
            });

            tokio::spawn(async move {
                tokio::select! {
                    _ = recv_task => {},
                    _ = send_task => {},
                }
            });
        }
    });

    send
}
