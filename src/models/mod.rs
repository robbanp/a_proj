pub mod merchant;
pub mod enums {
    use serde::{Serialize, Deserialize};
    use strum_macros::{EnumString, EnumVariantNames, Display};

    #[derive(Debug, sqlx::Type, Serialize, Deserialize, EnumString, EnumVariantNames, Display, Clone)]
    #[sqlx(type_name = "merchant_status", rename_all = "lowercase")] 
    #[serde(rename_all = "lowercase")]

    pub enum Status {
        #[strum(serialize = "pending")]
        Pending,
        #[strum(serialize = "approved")]
        Approved,
        #[strum(serialize = "denied")]
        Denied,
        #[strum(serialize = "sandbox")]
        Sandbox,
    }

    #[derive(Debug, Display)]
    pub enum HandlerError {
        DbError(String),
        ValidationError(String)
    }

    impl From<sqlx::Error> for HandlerError {
        fn from(err: sqlx::Error) -> Self {
          HandlerError::DbError(err.to_string()) 
        }
    }
}
