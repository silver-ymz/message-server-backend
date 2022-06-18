use crate::{
    database::DataBase,
    structure::{
        auth::{AuthBody, AuthError, Claims, Keys},
        user::User,
    },
};
use axum::{http::StatusCode, Extension, Json};
use jsonwebtoken::{encode, Header};
use std::sync::Arc;

pub async fn register(
    Json(user): Json<User>,
    Extension(database): Extension<Arc<DataBase>>,
) -> (StatusCode, String) {
    match database.insert_user(&user).await {
        Ok(_) => (StatusCode::CREATED, "register success".to_string()),
        Err(_) => (StatusCode::FORBIDDEN, "username already exist".to_string()),
    }
}

pub async fn login(
    Json(user): Json<User>,
    Extension(database): Extension<Arc<DataBase>>,
    Extension(keys): Extension<Arc<Keys>>,
) -> Result<Json<AuthBody>, AuthError> {
    database
        .find_user(&user)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    let claims = Claims {
        name: user.name,
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    let token = encode(&Header::default(), &claims, &keys.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}
