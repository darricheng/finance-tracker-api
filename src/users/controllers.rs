use super::super::db_config::{DB_NAME, USERS_COLLECTION_NAME};
use super::model::User;
use axum::{
    self,
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::{
    self,
    bson::{self, doc, Document},
    Client,
};

pub async fn add_user(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<User>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn get_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn update_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn delete_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
