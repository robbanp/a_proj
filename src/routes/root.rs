use axum::{Extension, http::StatusCode};
use axum::response::{Response, IntoResponse};
use axum::Json;
use sqlx::{Pool, Postgres};

pub async fn root_get(Extension(db): Extension<Pool<Postgres>>) -> Response {
    return (StatusCode::OK, Json("hi")).into_response();
}