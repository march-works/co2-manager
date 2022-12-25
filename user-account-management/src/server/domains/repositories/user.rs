use async_trait::async_trait;

use crate::server::domains::entities::user::User;

pub struct UserCreateFailed;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, name: String) -> Result<User, UserCreateFailed>;
}
