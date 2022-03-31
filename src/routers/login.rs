use crate::{database::DB, structure::user::User};
use axum::{http::StatusCode, Json};

pub async fn register(Json(user): Json<User>) -> StatusCode {
    match DB.get().await.insert_user(&user).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn login(Json(user): Json<User>) -> StatusCode {
    match DB.get().await {
        
    }
}
