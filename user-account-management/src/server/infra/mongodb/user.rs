use async_trait::async_trait;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::domains::{
    entities::user::User,
    errors::user::{UserError, UserErrorType, UserResult},
    repositories::user::UserRepository,
};

use super::get_handler;

#[derive(Debug, Serialize, Deserialize)]
struct UserDto {
    id: String,
    name: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id().into(),
            name: user.name().into(),
        }
    }
}

impl TryFrom<UserDto> for User {
    type Error = UserError;

    fn try_from(value: UserDto) -> UserResult<User> {
        User::new(value.id, value.name)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MongodbUserRepository;

#[async_trait]
impl UserRepository for MongodbUserRepository {
    async fn find_one(&self, id: String) -> UserResult<User> {
        let handler = get_handler()
            .await
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to connect to db"))?;
        let collection = handler.collection::<UserDto>("users");
        let user = collection
            .find_one(doc! { "id": &id }, None)
            .await
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to connect to db"))?;
        let user = user.ok_or_else(|| {
            UserError::new(UserErrorType::NotFound, format!("no user for {}", &id))
        })?;
        user.try_into()
    }

    async fn create_one(&self, name: String) -> UserResult<User> {
        let uuid = Uuid::new_v4().to_string();
        let user_dto = UserDto {
            id: uuid.clone(),
            name,
        };
        let handler = get_handler()
            .await
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to connect to db"))?;
        let collection = handler.collection::<UserDto>("users");
        collection
            .insert_one(&user_dto, None)
            .await
            .map_err(|_| UserError::new(UserErrorType::Duplicate, "already exists"))?;
        self.find_one(uuid).await
    }
}
