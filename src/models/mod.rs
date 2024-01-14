pub mod merchant;
pub mod enums {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, sqlx::Type, Serialize, Deserialize)]

    pub enum Status {
        Pending,
        Approved,
        Denied,
        Sandbox,
    }
}
