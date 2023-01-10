use mongodb::bson;
use serde::{Deserialize, Serialize};

// TODO: "Make category an enum: https://serde.rs/enum-representations.html"
// The enum variants should be set by the user

/// Base Transaction model
/// Represents the data structure of a transaction document in the database.
/// All fields are required. The derived structs are used for checking the validity of the data from users.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    pub date: bson::DateTime,
    pub category: String,
    pub value: f32,
    pub details: String,
}

impl Transaction {
    pub fn new(date: bson::DateTime, category: String, value: f32, details: String) -> Self {
        Transaction {
            date,
            category,
            value,
            details,
        }
    }
}

/// NewTransactionRequest model
/// Represents the data structure of data required to create a new transaction.
/// All fields are required. The derived structs are used for checking the validity of the data from users.
/// The date field is omitted as it is generated by the server.
#[derive(Debug, Deserialize, Serialize)]
pub struct NewTransactionRequest {
    pub category: String,
    pub value: f32,
    pub details: String,
}

/// TransactionQuery model
/// Represents the data structure of data required to query the database for transactions.
// TODO: Add support for querying by date range
// TODO: Add support for querying by category
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionQuery {
    pub date: String,
}
