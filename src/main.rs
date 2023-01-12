mod db_config;
mod transactions;

use axum::Router;

#[tokio::main]
async fn main() {
    // TODO: Build necessary environment variables
    // See: https://www.thorsten-hans.com/working-with-environment-variables-in-rust/

    // Build individual api routes
    let api_routes = Router::new().nest(
        "/transactions",
        transactions::routes::transaction_routes().await,
    );

    // Nest all api routes under an parent path "/api"
    let app = Router::new().nest("/api", api_routes);

    // Run it with hyper on localhost:10000
    // See for why PORT 10000: https://community.render.com/t/express-port-issue-on-web-service/4061/2
    axum::Server::bind(&"0.0.0.0:10000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
