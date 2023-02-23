use super::controllers;
use crate::db_config::get_mongodb_client;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{delete, get, patch, post},
    Router,
};
use std::env;
use tower_http::cors::CorsLayer;

pub async fn user_routes() -> Router {
    let mongodb_client = get_mongodb_client().await;

    // Get web app url from environment variables
    let web_app_url = env::var("WEB_APP_URL").expect("WEB_APP_URL must be set");

    let cors_layer = CorsLayer::new()
        .allow_origin(web_app_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/add", post(controllers::add_user))
        .route("/get_by_email", get(controllers::get_user_by_email))
        .route(
            "/update_categories",
            patch(controllers::update_user_categories),
        )
        .route("/delete_user", delete(controllers::delete_user))
        .with_state(mongodb_client)
        .layer(cors_layer)
}
