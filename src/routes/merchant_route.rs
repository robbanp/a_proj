use axum::{Extension, http::StatusCode};
use axum::response::{Response, IntoResponse};
use axum::Json;
use sqlx::{Pool, Postgres};

use crate::models::enums::Status;
use crate::models::merchant::Merchant;

pub async fn merchant_post(
    Extension(db): Extension<Pool<Postgres>>,
    Json(post): Json<Merchant>
) -> Response {


    let merchant_id = sqlx::query_scalar!(
        r#"insert into "merchants" (name, status) values ($1, $2) returning id"#,
        post.name,
        "pending",
    )
    .fetch_one(&db)
    .await;
    return (StatusCode::OK, Json(format!("found {:?}", merchant_id))).into_response();
}