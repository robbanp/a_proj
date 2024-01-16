use axum::body::Body;
use axum::extract::Path;
use axum::{Extension, http::StatusCode};
use axum::response::{Response, IntoResponse};

use axum::Json;
use serde_json::json;
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
        Status::Approved.to_string()
    )
    .fetch_one(&db)
    .await;

    match result {
        Ok(merchant) =>  return (StatusCode::OK, Json(merchant)).into_response(),    
        Err(_err) => {
            dbg!(&_err);
            let error = _err.as_database_error().map(|m| m.message());
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(error))).into_response()
        } 
      }
    }

    pub async fn merchant_update(
        Path(id): Path<i32>,
        Extension(db): Extension<Pool<Postgres>>,
        Json(post): Json<Merchant>
    ) ->  Response<Body>  {
    
        let mut query = String::from("update merchants SET ");
        if let Some(name) = post.name {
            query = format!(r#"{} name = '{}' "#, query, name);
        }
        if let Some(status) = post.status {
            query = format!("{}, status = {} ", query, status);
        }
        if let Some(md) = post.metadata {
            query = format!("{}, metadata = {} ", query, md);
        }
        query = format!("{} where id = {} ", query, id);
        query = format!("{}  {}", query, r#"returning id, name, created_at, updated_at, metadata, status as "status!: enums::Status""#);

        dbg!(&query);
        let result = sqlx::query_as::<_, Merchant>(
            &query,
        )
        .fetch_one(&db)
        .await;
    
        match result {
            Ok(merchant) =>  return (StatusCode::OK, Json(merchant)).into_response(),    
            Err(_err) => {
                dbg!(&_err);
                let error = _err.as_database_error().map(|m| m.message());
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(error))).into_response()
            } 
          }
        }
        
    pub async fn merchant_list(
        Extension(db): Extension<Pool<Postgres>>
    ) ->  Response<Body>  {
    
        let result = sqlx::query_as!(Merchant,
            r#"SELECT id, name, created_at, updated_at, metadata, status as "status!: enums::Status" FROM merchants"#,
        )
        .fetch_all(&db)
        .await;
    
        match result {
            Ok(merchants) =>  return (StatusCode::OK, Json(merchants)).into_response(),    
            Err(_err) => {
                dbg!(&_err);
                let error = _err.as_database_error().map(|m| m.message());
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(error))).into_response()
            } 
          }
        }
    