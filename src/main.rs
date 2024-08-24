mod api;

use api::hello::hello;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
