use mongodb::Client;
use std::env;

pub async fn get_mongodb_client() -> Client {
    // Get mongo uri from environment variables
    let mongo_uri = match env::var("MONGO_URI") {
        Ok(uri) => uri,
        Err(_) => {
            println!("MONGO_URI must be set");
            String::new()
        }
    };
    Client::with_uri_str(mongo_uri).await.unwrap()
}
