use axum::Router;
use axum::http::Request;
use axum::body::Body;
use axum::http::Response;
use dotenvy_macro::dotenv;
use dotenvy::dotenv;
use psp_core::models::{merchant::Merchant, enums::Status};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use fake::Fake;
use fake::locales::*;
use fake::faker::company::raw::*;


pub async fn init_db() -> Pool<Postgres> {
    dotenv().ok();
    let conn_str = dotenv!("DATABASE_URL");
    let conn_str_test = dotenv!("DATABASE_URL_TEST");
    let test_database_name = format!("psp_core_test");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_str).await.expect("msg");

    sqlx::query(&format!("DROP DATABASE IF EXISTS {};", test_database_name))
    .execute(&db)
    .await
    .expect("Failed to DROP test database");

    sqlx::query(&format!("CREATE DATABASE {};", test_database_name))
    .execute(&db)
    .await
    .expect("Failed to CREATE test database");

 
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_str_test).await.expect("can not connect to test db");

    sqlx::migrate!("./migrations/").run(&db).await.unwrap();

    db
}

pub async fn kill_db(db: Pool<Postgres>) {
    sqlx::query(&format!("DROP DATABASE IF EXISTS test_database"))
        .execute(&db)
        .await.expect("could not drop");
}

pub async fn get_req(method: &str, uri: &str, body: String) -> Request<String> {
    Request::builder().header("content-type", "application/json").method(method).uri(uri).body(body).unwrap()
}

pub fn merchant_fac() -> Merchant {
    Merchant {
        name: Some(CompanyName(EN).fake()),  
        id: None,
        created_at: None,
        updated_at: None,
        status: Some(Status::Pending),
        metadata: None,
    }
}

pub async fn from_response<T: for<'a> serde::de::Deserialize<'a>>(app: axum::Router, req: Request<String>) -> T {
    let response: Response<Body> = app
    .oneshot(req)
    .await
    .unwrap();
    let res_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(res_bytes.to_vec()).unwrap();
    let created_model: T = serde_json::from_str(&body_str).unwrap();
    created_model
}