use axum::Extension;
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
mod root;
use root::root_get;


pub fn create_routes(db: Pool<Postgres>) -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new()
    .route("/", get(root_get))
    .layer(cors) // affect allroutes above
    .layer(Extension(db))
}