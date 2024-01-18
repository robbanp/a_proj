//! Run with
//!
//! ```not_rust
//! cargo test -p example-testing
//! ```

use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    routing::{get, post},
    Json, Router,
};
use dotenvy_macro::dotenv;
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::postgres::PgPoolOptions;

fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        .route(
            "/requires-connect-into",
            get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move { format!("Hi {addr}") }),
        )
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
}

async fn test_db() -> Pool<Postgres> {
    dotenv().ok();
    let conn_str = dotenv!("DATABASE_URL");
    let test_database_name = format!("test_database");


    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_str).await.expect("msg");

    sqlx::query(&format!("CREATE DATABASE {}", test_database_name))
    .execute(&db)
    .await
    .expect("Failed to create test database");

 
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_str).await.expect("msg");

    sqlx::migrate!("./migrations/").run(&db).await.unwrap();

    db
}

async fn kill_db(db: Pool<Postgres>) {
    sqlx::query(&format!("DROP DATABASE test_database"))
        .execute(&db)
        .await;
        return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn hello_world() {
        let db = test_db().await;
        let app = psp_core::routes::create_routes(db.clone());

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
        kill_db(db);
    }

    }
