use axum::{middleware::from_fn, routing::get, Router};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
mod routers;
mod web_socket;
use routers::{index, ws::ws_handler};
mod structure;
use structure::state::State;
mod middlewares;
use middlewares::global_middlewire;
// mod database;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .expect("Cannot find the env var PORT")
        .parse()
        .expect("the env var PORT cannot parse to u16");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let state = Arc::new(Mutex::new(State::new()));

    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(ws_handler))
        .layer(from_fn({
            move |req, next| global_middlewire(req, next, state.clone())
        }));

    println!("Server listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
