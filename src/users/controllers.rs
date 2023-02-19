use super::super::db_config::{DB_NAME, USERS_COLLECTION_NAME};
use super::model::{NewUserRequest, User, UserCategories};
use axum::{
    self,
    extract::{self, State},
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use mongodb::Collection;
use mongodb::{
    self,
    bson::{self, doc, Document},
    Client,
};
use serde::Deserialize;

pub async fn add_user(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<NewUserRequest>,
) -> impl IntoResponse {
    let new_user = User::new(
        json_payload.email,
        json_payload.firebase_id,
        Vec::new(),
        // String::new(),
    );
    // Serialize the struct to a bson document
    let bson_document = match bson::to_document(&new_user) {
        Ok(document) => document,
        Err(err) => {
            println!("Error converting user to bson document: {err:?}");
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

/// The query string for the get_user_by_email route
#[derive(Deserialize)]
pub struct EmailQuery {
    email: String,
}

pub async fn get_user_by_email(
    extract::State(state): State<Client>,
    extract::Query(email_query): extract::Query<EmailQuery>,
) -> response::Result<Json<User>, StatusCode> {
    let collection = state.database(DB_NAME).collection(USERS_COLLECTION_NAME);
    let result = collection
        .find_one(doc! {"email": email_query.email}, None)
        .await;

    let res_doc = match result {
        Ok(document) => document,
        Err(err) => {
            println!("Error finding user by email: {err:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let bson_user = match res_doc {
        Some(doc) => doc,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let user: User = match bson::from_document(bson_user) {
        Ok(user) => user,
        Err(err) => {
            println!("Error converting bson document to user: {err:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(user))
}

pub async fn update_user_categories(
    extract::State(state): State<Client>,
    extract::Json(user_categories): extract::Json<UserCategories>,
) -> impl IntoResponse {
    let collection: Collection<Document> =
        state.database(DB_NAME).collection(USERS_COLLECTION_NAME);
    let result = collection
        .update_one(
            doc! {"email": user_categories.email},
            doc! {"$set": {"categories": user_categories.categories}},
            None,
        )
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
