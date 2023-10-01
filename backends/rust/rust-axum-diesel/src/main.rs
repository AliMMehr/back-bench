use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::endpoints::{
    hello::hello_world, return_large_json::large_json_endpoint, spin_sleep::spin_sleep,
    thread_sleep::thread_sleep,
};

mod endpoints;

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt().finish();

    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/large_json", get(large_json_endpoint))
        .route("/thread_sleep", get(thread_sleep))
        .route("/spin_sleep", get(spin_sleep));

    let app = app.layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run it with hyper on localhost:3000
    println!("Starting Server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
