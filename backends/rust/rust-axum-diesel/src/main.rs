use axum::{routing::get, Router};

use crate::endpoints::{hello::hello_world, return_large_json::large_json_endpoint};

mod endpoints;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/large_json", get(large_json_endpoint));

    // run it with hyper on localhost:3000
    println!("Starting Server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
