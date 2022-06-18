use crate::structure::{state::AppState, user::User};
use axum::{
    body::{boxed, Body, BoxBody},
    http::{header, Request, Response, StatusCode},
};
use std::sync::Arc;
use tower_http::auth::AuthorizeRequest;

#[derive(Clone, Copy)]
pub struct MyAuth;

impl<B> AuthorizeRequest<B> for MyAuth {
    type ResponseBody = BoxBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        match check_auth(request) {
            Ok(username) => {
                println!("{} complete auth", username);

                request
                    .extensions_mut()
                    .get_mut::<AppState>()
                    .unwrap()
                    .set_auth(username.clone());

                Ok(())
            }

            Err(AuthError::MissingHeader) => {
                println!("need auth");
                Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::WWW_AUTHENTICATE, "Basic")
                    .body(boxed(Body::empty()))
                    .unwrap())
            }

            _ => {
                println!("break");
                Err(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(boxed(Body::empty()))
                    .unwrap())
            }
        }
    }
}

fn check_auth<B>(request: &mut Request<B>) -> Result<String, AuthError> {
    // Check the 'Authorization' header for a `Basic` token.
    let auth = request
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AuthError::MissingHeader)?
        .to_str()
        .map_err(|_| AuthError::InvalidHeader)?;

    if !auth.starts_with("Basic ") {
        return Err(AuthError::InvalidCredentials);
    }

    let auth = &auth["Basic ".len()..];

    let (username, password) = base64::decode(auth)
        .map(|s| String::from_utf8(s).unwrap())
        .map_err(|_| AuthError::InvalidBase64)?
        .split_once(':')
        .unwrap();

    let ex = request
        .extensions()
        .get::<Arc<AppState>>()
        .unwrap()
        .database
        .clone();

    let username = username.clone();
    let password = password.clone();
    let th = std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                ex.find_user(&User {
                    name: username.to_string(),
                    password: password.to_string(),
                })
                .await
                .is_ok()
            })
    });
    if th.join().unwrap() {
        return Ok(username.to_string());
    }

    Err(AuthError::InvalidCredentials)
}

enum AuthError {
    MissingHeader,
    InvalidHeader,
    InvalidBase64,
    InvalidCredentials,
}
