mod common;
use common::{kill_db, init_db};
use psp_core::models::merchant::Merchant;
#[cfg(test)]
mod tests {
    use super::*;
    use axum::
        http::StatusCode
    ;


    // get an empty list of merchants
    #[tokio::test]
    async fn get_merchants_empty() {
        let db = init_db().await;
        let app = psp_core::routes::create_routes(db.clone());
        let req = common::get_req("GET", "/merchants", String::new());
        let response = common::get_response(app, req).await;
        assert_eq!(response.status(), StatusCode::OK);

        let list: Vec<Merchant> = common::from_response::<Vec<Merchant>>(response).await;
        assert_eq!(list.len(), 0);
        dbg!(list);
        kill_db(db).await;
    }

    // create a merchant with a name and get it back
    #[tokio::test]
    async fn create_merchants() {
        let db = init_db().await;
        let app = psp_core::routes::create_routes(db.clone());
        let body = common::merchant_fac();
        let json = serde_json::to_value(&body).unwrap().to_string();
        let req = common::get_req("POST", "/merchants", json);
        let response = common::get_response(app, req).await;
        assert_eq!(response.status(), StatusCode::OK);

        let m: Merchant = common::from_response::<Merchant>(response).await;
        assert_eq!(m.name, body.name);
        kill_db(db).await;
    }

}
