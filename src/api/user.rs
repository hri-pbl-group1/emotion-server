use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

pub async fn get_user(
    State(db): State<Arc<Mutex<PgPool>>>,
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    let pool = db.lock().await;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id,)
        .fetch_one(&*pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn create_user(
    State(db): State<Arc<Mutex<PgPool>>>,
    user: Json<User>,
) -> Result<(), (StatusCode, String)> {
    let pool = db.lock().await;

    let _result = sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user.id,
        user.name,
    )
    .execute(&*pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user".to_string(),
        )
    })?;

    Ok(())
}
