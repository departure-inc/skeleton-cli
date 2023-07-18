use crate::infrastructure::response::message_response::MessageResponse;
use axum::{response::IntoResponse, Json};

pub async fn root() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn healthcheck() -> impl IntoResponse {
    Json(MessageResponse {
        message: "ok".to_string(),
    })
}
