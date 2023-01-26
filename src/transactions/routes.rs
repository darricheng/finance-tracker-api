use super::super::db_config::MONGO_URI;
use super::controllers;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use mongodb::Client;

pub async fn transaction_routes() -> Router {
    let mongodb_client = Client::with_uri_str(MONGO_URI).await.unwrap();

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
}
