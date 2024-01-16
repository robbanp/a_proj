use axum::Extension;
use axum::routing::{post, put};
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
mod root;
use root::root_get;

mod merchant_route;
use self::merchant_route::{merchant_post, merchant_list, merchant_update, merchant_get};

pub fn create_routes(db: Pool<Postgres>) -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new()
    .route("/", get(root_get))
    .route("/merchants", post(merchant_post))
    .route("/merchants", get(merchant_list))
    .route("/merchants/:id", put(merchant_update))
    .route("/merchants/:id", get(merchant_get))
    .layer(cors) // affect allroutes above
    .layer(Extension(db))
}