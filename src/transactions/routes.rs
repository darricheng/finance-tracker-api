use super::controllers;
use crate::db_config::get_mongodb_client;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;

// TODO: Add middleware for accepting requests from anywhere (See users/routes.rs for more info)
// So that I can eventually call the API from an iOS shortcut

pub async fn transaction_routes() -> Router {
    let mongodb_client = get_mongodb_client().await;

    // TODO: Use environment variables for the allow_origin URL, so that it can be changed in production
    let cors_layer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/get_all", get(controllers::get_transactions))
        .route("/add", post(controllers::add_transaction))
        .route(
            "/get_by_date_range",
            get(controllers::get_transactions_by_date_range),
        )
        .route("/update", put(controllers::update_transaction))
        .route("/delete", delete(controllers::delete_transaction))
        .with_state(mongodb_client)
        .layer(cors_layer)
}
