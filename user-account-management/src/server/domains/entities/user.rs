use std::sync::Arc;

use crate::server::domains::repositories::user::{UserCreateFailed, UserRepository};

pub struct UserID(pub String);

pub struct UserName(pub String);

pub struct User {
    pub id: UserID,
    pub name: UserName,
}

impl User {
    fn validate_name(name: &str) -> bool {
        name.len() <= 10
    }

    pub fn new(id: String, name: String) -> Option<Self> {
        if Self::validate_name(&name) {
            Some(User {
                id: UserID(id),
                name: UserName(name),
            })
        } else {
            None
        }
    }

    pub async fn create(
        name: String,
        repository: Arc<dyn UserRepository>,
    ) -> Result<Self, UserCreateFailed> {
        repository.create(name).await
    }
}
