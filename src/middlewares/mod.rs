use crate::structure::state::State;
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use std::sync::{Arc, Mutex};

// TODO: user tower to rewrite middleware
pub async fn global_middlewire(
    mut req: Request<Body>,
    next: Next<Body>,
    state: Arc<Mutex<State>>,
) -> impl IntoResponse {
    //let method = req.method().as_str();
    //let uri = req.uri().path().clone();
    req.extensions_mut().insert(state.clone());
    {
        let visit = &mut state.lock().unwrap().visit_count;
        *visit += 1;
        //log::info!("NO.{}: {} {}", visit, method, uri);
    }
    next.run(req).await
}
