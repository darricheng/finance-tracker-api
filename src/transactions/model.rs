use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    pub date: String,
    pub category: String,
    pub value: f32,
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionQuery {
    pub date: String,
}
