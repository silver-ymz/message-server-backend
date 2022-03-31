use tokio::sync::{mpsc, broadcast};
use lazy_static::lazy_static;
use axum::extract::ws::{WebSocket, Message};
use std::sync::Arc;
use futures::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}, TryStreamExt, future};

lazy_static! {
    pub static ref WEBSOCKET_SENDER: Arc<mpsc::Sender<WebSocket>> = {
        let (send, mut recv): (mpsc::Sender<WebSocket>, mpsc::Receiver<WebSocket>) = mpsc::channel(10);
        tokio::spawn(async move {
            let (tx, _) = broadcast::channel::<String>(20);
            while let Some(socket) = recv.recv().await {
                let (sender, receiver) = socket.split::<Message>();
                let tx0 = tx.clone();
                let rx0 = tx0.subscribe();
                tokio::spawn(read(receiver, tx0));
                tokio::spawn(write(sender, rx0));
            }
        });
        Arc::new(send)
    };
}

async fn read(receiver: SplitStream<WebSocket>, tx: broadcast::Sender<String>) {
    receiver.try_for_each(move |msg| {
        match msg {
            Message::Text(s) => tx.send(s).unwrap(),
            _ => 0,
        };
        future::ok(())
    }).await.unwrap();
}

async fn write(mut sender: SplitSink<WebSocket, Message>, mut rx: broadcast::Receiver<String>) {
    loop {
        let s = rx.recv().await.unwrap();
        sender.send(Message::Text(s)).await.unwrap();
    }
}