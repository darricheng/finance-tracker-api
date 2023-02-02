use super::controllers;
use crate::db_config::get_mongodb_client;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub async fn user_routes() -> Router {
    let mongodb_client = get_mongodb_client().await;

    Router::new()
        .route("/add_user", post(controllers::add_user))
        .route("/get_user", get(controllers::get_user))
        .route("/update_user", patch(controllers::update_user))
        .route("/delete_user", delete(controllers::delete_user))
        .with_state(mongodb_client)
}
