use mongodb::Client;
use std::env;

pub async fn get_mongodb_client() -> Client {
    // Get mongo uri from environment variables
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    Client::with_uri_str(mongo_uri).await.unwrap()
}
