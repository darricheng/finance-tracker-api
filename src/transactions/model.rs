use mongodb::bson;
use serde::{Deserialize, Serialize};

// TODO: "Make category an enum: https://serde.rs/enum-representations.html"
// The enum variants should be set by the user

/// Transaction model
/// Represents the data structure of a transaction document in the database.
/// All fields are required. The derived structs are used for checking the validity of the data from users.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    // serializes chrono datetime as a BSON datetime, see: https://docs.rs/bson/latest/bson/struct.DateTime.html
    // #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    // TODO: Uncommenting the above line results in an inability to send json requests as an ISODate string
    pub date: chrono::DateTime<chrono::Utc>,
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
