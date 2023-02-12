use super::controllers;
use crate::db_config::get_mongodb_client;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::cors::CorsLayer;

pub async fn user_routes() -> Router {
    let mongodb_client = get_mongodb_client().await;

    // TODO: Use environment variables for the allow_origin URL, so that it can be changed in production
    let cors_layer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/add_user", post(controllers::add_user))
        .route("/get_user_by_email", get(controllers::get_user_by_email))
        .route(
            "/update_user_categories",
            patch(controllers::update_user_categories),
        )
        .route("/delete_user", delete(controllers::delete_user))
        .with_state(mongodb_client)
        .layer(cors_layer)
}
