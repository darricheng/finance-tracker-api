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
        .route("/get_all", get(controllers::get_transactions))
        .route("/add", post(controllers::add_transaction))
        .route(
            "/get_by_date_range",
            get(controllers::get_transactions_by_date_range),
        )
        .route("/update", post(controllers::update_transaction))
        .route("/delete", post(controllers::delete_transaction))
        .with_state(mongodb_client)
}
