use dotenvy_macro::dotenv;
use dotenvy::dotenv;
mod routes;
use sqlx::postgres::PgPoolOptions;
mod models;

pub async fn run() {
    dotenv().ok();
    let conn_str = dotenv!("DATABASE_URL");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_str)
        .await.expect("cannot connect to db");

    let _ = sqlx::migrate!().run(&db).await;
    
    let app = routes::create_routes(db);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

