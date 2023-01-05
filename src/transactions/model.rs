use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Transaction {
    pub date: String,
    pub value: i32,
    pub details: String,
}
