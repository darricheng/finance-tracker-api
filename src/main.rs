use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, Rust World!" }));

    // run it with hyper on localhost:10000
    // See for why PORT 10000: https://community.render.com/t/express-port-issue-on-web-service/4061/2
    axum::Server::bind(&"0.0.0.0:10000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
