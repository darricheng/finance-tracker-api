use super::super::db_config::{COLLECTION_NAME, DB_NAME};
use super::model::{Transaction, TransactionQuery};
use axum::{
    extract::{self, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::bson::Document;
use mongodb::{bson::doc, Client};

pub async fn add_transaction(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<Transaction>,
) -> impl IntoResponse {
    let collection = state.database(DB_NAME).collection(COLLECTION_NAME);
    let result = collection.insert_one(json_payload, None).await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_transactions(extract::State(state): State<Client>) -> impl IntoResponse {
    let collection = state
        .database(DB_NAME)
        .collection::<Document>(COLLECTION_NAME); // Why add <Document> type annotation https://stackoverflow.com/a/71439769

    let result = collection.find(None, None).await;

    // TODO: Return a json if found
}

pub async fn get_transactions_by_date(
    extract::State(state): State<Client>,
    params: Query<TransactionQuery>,
) -> impl IntoResponse {
    println!("{:?}", params.0);
    let filter = Some(params.0);

    // TODO: Implement function!
}
