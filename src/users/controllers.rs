use super::super::db_config::{DB_NAME, USERS_COLLECTION_NAME};
use super::model::{NewUserRequest, User};
use axum::{
    self,
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::{
    self,
    bson::{self, doc, Document},
    Client,
};

pub async fn add_user(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<NewUserRequest>,
) -> impl IntoResponse {
    let new_user = User::new(
        json_payload.email,
        json_payload.firebase_id,
        Vec::new(),
        String::new(),
    );
    // Serialize the struct to a bson document
    let bson_document = match bson::to_document(&new_user) {
        Ok(document) => document,
        Err(err) => {
            println!("Error converting user to bson document: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Insert the bson document into the database
    let collection = state.database(DB_NAME).collection(USERS_COLLECTION_NAME);
    let result = collection.insert_one(bson_document, None).await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn update_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn delete_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
