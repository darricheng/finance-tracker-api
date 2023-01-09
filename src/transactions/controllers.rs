use super::super::db_config::{COLLECTION_NAME, DB_NAME};
use super::model::{Transaction, TransactionQuery};
use axum::Json;
use axum::{
    self,
    extract::{self, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::stream::TryStreamExt;
use mongodb::{
    self,
    bson::{self, Document},
    Client,
};

pub async fn add_transaction(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<Transaction>,
) -> impl IntoResponse {
    let collection = state.database(DB_NAME).collection(COLLECTION_NAME);

    println!("json_payload: {:?}", json_payload);

    // TODO: Add the date field to the json_payload

    let result = collection.insert_one(json_payload, None).await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_transactions(
    extract::State(state): State<Client>,
) -> axum::response::Result<Json<Vec<Transaction>>, StatusCode> {
    let collection = state
        .database(DB_NAME)
        .collection::<Document>(COLLECTION_NAME); // Why add <Document> type annotation https://stackoverflow.com/a/71439769

    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            println!("Error: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut result_vec: Vec<Transaction> = Vec::new();

    // Iterate over the results of the cursor.
    while let Ok(Some(transaction)) = cursor.try_next().await {
        match bson::from_bson(bson::Bson::Document(transaction)) {
            Ok(val) => result_vec.push(val),
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    // Return a json of the result vector
    Ok(axum::Json(result_vec))
}

pub async fn get_transactions_by_date(
    extract::State(state): State<Client>,
    params: Query<TransactionQuery>,
) -> impl IntoResponse {
    println!("{:?}", params.0);
    let filter = Some(params.0);

    // TODO: Implement function!
}
