use super::super::db_config::{COLLECTION_NAME, DB_NAME};
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

pub async fn add_user() -> impl IntoResponse {
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
