use std::error::Error;

use axum::body::Body;
use axum::{Extension, http::StatusCode};
use axum::response::{Response, IntoResponse};

use axum::Json;
use sqlx::{Pool, Postgres};

use crate::models::enums::{Status, self};
use crate::models::merchant::Merchant;

pub async fn merchant_post(
    Extension(db): Extension<Pool<Postgres>>,
    Json(post): Json<Merchant>
) ->  Response<Body>  {

    let result = sqlx::query_as!(Merchant,
        r#"insert into merchants (name, status) values ($1, $2) returning id, name, created_at, updated_at, metadata, status as "status!: enums::Status""#,
        post.name,
        Status::Approved as _
    )
    .fetch_one(&db)
    .await;

    match result {
        Ok(merchant) =>  return (StatusCode::OK, Json(merchant)).into_response(),    
        Err(_err) => {
            let error = _err.as_database_error().map(|m| m.message());
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(error)).into_response()
        } 
      }
    }
 