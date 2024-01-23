mod common;
use common::{kill_db, init_db};
use psp_core::models::merchant::Merchant;
use psp_core::models::enums::Status;
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Response;
    use axum::
        http::StatusCode
    ;
    use http_body_util::BodyExt;    
    use tower::ServiceExt;
    use fake::Fake;
    use fake::locales::*;
    use fake::faker::company::raw::*;


    #[tokio::test]
    async fn get_merchants() {
        let db = init_db().await;
        let app = psp_core::routes::create_routes(db.clone());
        let req = common::get_req("GET", "/merchants", String::new()).await;
        let response = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let res_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(res_bytes.to_vec()).unwrap();
        let list: Vec<Merchant> = serde_json::from_str(&body_str).unwrap();
        assert_eq!(list.len(), 0);
        dbg!(list);
        kill_db(db).await;
    }

    #[tokio::test]
    async fn create_merchants() {
        let db = init_db().await;
        let app = psp_core::routes::create_routes(db.clone());
        let body = common::merchant_fac();
        let j = serde_json::to_value(&body).unwrap().to_string();
        let req = common::get_req("POST", "/merchants", j).await;
        let response: Response<Body> = app
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let res_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(res_bytes.to_vec()).unwrap();
        let created_merchant: Merchant = serde_json::from_str(&body_str).unwrap();
        dbg!(&created_merchant);
        assert_eq!(created_merchant.name, body.name);
        kill_db(db).await;
    }

}
