use crate::{database::connect_db, web_socket::create_websocket_sender, structure::auth::Keys};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use routers::{
    index,
    login::{login, register},
    ws::ws_handler,
};
use std::{net::SocketAddr, sync::Arc};
//use structure::state::AppState;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

mod middlewares;
mod database;
mod routers;
mod structure;
mod web_socket;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .expect("Cannot find the env var PORT")
        .parse()
        .expect("the env var PORT cannot parse to u16");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    //let app_state = Arc::new(AppState::new(create_websocket_sender(), connect_db().await));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/", get(index))
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(Arc::new(connect_db().await)))
                .layer(Extension(create_websocket_sender()))
                .layer(Extension(Arc::new(Keys::new())))
                .layer(CompressionLayer::new()),
        );

    println!("Server listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
