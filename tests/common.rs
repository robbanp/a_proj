use dotenvy_macro::dotenv;
use dotenvy::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};


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
