use super::super::db_config::{COLLECTION_NAME, DB_NAME};
use super::model::{NewTransactionRequest, ReturnTransaction, Transaction, TransactionDateQuery};
use axum::Json;
use axum::{
    self,
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{
    self,
    bson::{self, Document},
    Client,
};

pub async fn add_transaction(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<NewTransactionRequest>,
) -> impl IntoResponse {
    // Add the date field to the json_payload
    // Generate a new DateTime object with the current time
    // We use UTC time for ease of use
    // When we convert the DateTime object to a bson DateTime object, it will be converted to UTC time anyway
    let date = chrono::Utc::now();
    println!("date: {:?}", date);
    // Convert the DateTime object to a bson DateTime object
    let date = bson::DateTime::from_chrono(date);
    // Create an instance of a new struct that includes the json data and the date
    let transaction = Transaction::new(
        date,
        json_payload.category,
        json_payload.value,
        json_payload.details,
    );
    println!("transaction: {:?}", transaction);

    // Serialize the new struct to a bson document
    let bson_document = match bson::to_document(&transaction) {
        Ok(document) => document,
        Err(err) => {
            println!("Error converting transaction to bson document: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Insert the bson document into the database
    let collection = state.database(DB_NAME).collection(COLLECTION_NAME);
    let result = collection.insert_one(bson_document, None).await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_transactions(
    extract::State(state): State<Client>,
) -> axum::response::Result<Json<Vec<ReturnTransaction>>, StatusCode> {
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

    // Convert the result vector to a vector of ReturnTransaction structs
    let result_vec: Vec<ReturnTransaction> = result_vec
        .iter()
        // .map(|transaction| ReturnTransaction::into(transaction))
        .map(ReturnTransaction::from_transaction)
        .collect();

    // Return a json of the result vector
    Ok(axum::Json(result_vec))
}

pub async fn get_transactions_by_date_range(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<TransactionDateQuery>,
) -> axum::response::Result<Json<Vec<ReturnTransaction>>, StatusCode> {
    // The function searches by time to the hour as the dates are stored in UTC time
    // but the user would be searching by their local time
    let bson_start_date = bson::DateTime::from_chrono(json_payload.start_date);
    let bson_end_date = bson::DateTime::from_chrono(json_payload.end_date);

    let collection = state
        .database(DB_NAME)
        .collection::<Document>(COLLECTION_NAME);

    let filter = doc! {
        "date": {
            "$gte": bson_start_date,
            "$lte": bson_end_date
        }
    };

    let mut cursor = match collection.find(filter, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            println!("Error: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut result_vec: Vec<Transaction> = Vec::new();

    // Iterate over the results of the cursor.
    while let Ok(Some(transaction)) = cursor.try_next().await {
        match bson::from_bson::<Transaction>(bson::Bson::Document(transaction)) {
            Ok(val) => result_vec.push(val),
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    // Convert the result vector to a vector of ReturnTransaction structs
    let result_vec: Vec<ReturnTransaction> = result_vec
        .iter()
        // .map(|transaction| ReturnTransaction::into(transaction))
        .map(ReturnTransaction::from_transaction)
        .collect();

    // Return a json of the result vector
    Ok(axum::Json(result_vec))
}

pub async fn update_transaction() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn delete_transaction() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
