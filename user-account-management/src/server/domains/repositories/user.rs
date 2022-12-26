use async_trait::async_trait;

use crate::server::domains::{entities::user::User, errors::user::UserResult};

#[mockall::automock]
#[async_trait]
pub trait UserRepository {
    async fn find_one(&self, id: String) -> UserResult<User>;

    async fn create_one(&self, name: String) -> UserResult<User>;
}
