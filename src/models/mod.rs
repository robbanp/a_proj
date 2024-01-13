pub mod merchant;

pub mod enums {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Denied,
        Sandbox,
    }
}
