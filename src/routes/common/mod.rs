use axum::{http::StatusCode, Json};

use crate::models::enums::HandlerError;

// Error handling function
pub fn handle_error(err: sqlx::Error) -> (StatusCode, Json<String>) {
    let error = HandlerError::from(err);
    let status = match error {
        HandlerError::DbError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        HandlerError::ValidationError(_) => StatusCode::BAD_REQUEST
    };

    (status, Json(error.to_string()))
}