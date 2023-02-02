use serde::{Deserialize, Serialize};

/// Base User model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub categories: Vec<String>,
    pub api_key: String,
    pub firebase_id: String,
    pub email: String,
}
