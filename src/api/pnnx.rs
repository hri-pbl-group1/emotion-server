use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPnnx {
    pub value: f64,
}

pub async fn post_pnnx(
    State(db): State<Arc<Mutex<PgPool>>>,
    Path(user_id): Path<String>,
    data: Json<PostPnnx>,
) -> Result<(), (StatusCode, String)> {
    let pool = db.lock().await;

    let _user = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id,)
        .fetch_one(&*pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let _result = sqlx::query!(
        "INSERT INTO pnnx (id, value) VALUES ($1, $2)",
        user_id,
        data.value,
    )
    .execute(&*pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create pnnx".to_string(),
        )
    })?;

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
pub struct GetPnnxQuery {
    pub num: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GetPnnx {
    // pub id: String,
    pub value: f64,
}

pub async fn get_pnnx(
    State(db): State<Arc<Mutex<PgPool>>>,
    Path(id): Path<String>,
    Query(query): Query<GetPnnxQuery>,
) -> Result<Json<Vec<GetPnnx>>, (StatusCode, String)> {
    let pool = db.lock().await;

    let _user = sqlx::query!("SELECT * FROM users WHERE id = $1", id,)
        .fetch_one(&*pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let pnnx = sqlx::query_as!(
        GetPnnx,
        "SELECT value FROM pnnx WHERE id = $1 ORDER BY timestamp DESC LIMIT $2",
        id,
        query.num
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get pnnx".to_string(),
        )
    })?;

    Ok(Json(pnnx))
}
