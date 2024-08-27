mod api;

use std::sync::Arc;

use api::hello::hello;
use api::pnnx::{get_pnnx, post_pnnx};
use api::user::{create_user, get_user};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432")
        .await?;

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user))
        .route("/pnnx/:id", get(get_pnnx).post(post_pnnx))
        .with_state(Arc::new(Mutex::new(pool)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
