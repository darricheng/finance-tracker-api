use super::super::db_config::MONGO_URI;
use super::controllers;
use axum::{
    routing::{get, post},
    Router,
};
use mongodb::Client;

pub async fn transaction_routes() -> Router {
    let mongodb_client = Client::with_uri_str(MONGO_URI).await.unwrap();

    Router::new()
        .route("/", get(|| async { "Hello base path!" }))
        .route("/get-all", get(controllers::get_transactions))
        .route("/add", post(controllers::add_transaction))
        .with_state(mongodb_client)
}
