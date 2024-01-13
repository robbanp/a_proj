use axum::{Extension, http::StatusCode};
use axum::response::{Response, IntoResponse};
use axum::Json;
use sqlx::{Pool, Postgres};

pub async fn root_get(Extension(db): Extension<Pool<Postgres>>) -> Response {
    let merchant_count: Result<i64, sqlx::Error> = sqlx::query_scalar(
        "select count(*) from merchants")
    .fetch_one(&db)
    .await;
    
    return (StatusCode::OK, Json(format!("found {:?} merchants", merchant_count.unwrap()))).into_response();
}