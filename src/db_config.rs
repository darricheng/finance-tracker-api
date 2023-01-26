use mongodb::Client;

// TODO: Add support for environment variables (See main.rs for more info)

pub const MONGO_URI: &str = "mongodb://localhost:27017/";
pub const DB_NAME: &str = "financeTrackerApp";
pub const COLLECTION_NAME: &str = "testTransactions";

pub async fn get_mongodb_client() -> Client {
    Client::with_uri_str(MONGO_URI).await.unwrap()
}
