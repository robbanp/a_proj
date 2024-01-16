use sqlx::types::chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use validator::Validate;
use super::enums;

#[derive(Serialize, Deserialize, Debug, Validate, FromRow, Clone)]
pub struct Merchant {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub status: Option<enums::Status>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>
}

