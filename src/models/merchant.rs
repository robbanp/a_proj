use serde::{Serialize, Deserialize};
use validator::Validate;

use super::enums;


#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Merchant {
    pub id: Option<i32>,
    pub name: String,
    pub status: Option<enums::Status>,
}