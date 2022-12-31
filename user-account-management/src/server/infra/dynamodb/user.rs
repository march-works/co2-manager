use std::env;

use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
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
pub struct DynamodbUserRepository;

#[async_trait]
impl UserRepository for DynamodbUserRepository {
    async fn find_one(&self, id: String) -> UserResult<User> {
        let table_name = env::var("USERS_NAME")
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to parse env"))?;
        let handler = get_handler().await;
        let item = handler
            .get_item()
            .table_name(&table_name)
            .key("id", AttributeValue::S(id.clone()))
            .send()
            .await
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to connect to db"))?;
        let not_found = UserError::new(UserErrorType::NotFound, format!("no user for {}", &id));
        if let Some(user) = item.item() {
            let id = user
                .get("id")
                .map(|v| v.as_s())
                .ok_or_else(|| not_found.clone())?
                .map_err(|_| not_found.clone())?;
            let name = user
                .get("name")
                .map(|v| v.as_s())
                .ok_or_else(|| not_found.clone())?
                .map_err(|_| not_found.clone())?;
            User::new(id, name)
        } else {
            Err(not_found)
        }
    }

    async fn create_one(&self, name: String) -> UserResult<User> {
        let table_name = env::var("USERS_NAME")
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to parse env"))?;
        let uuid = Uuid::new_v4().to_string();
        let user_dto = UserDto {
            id: uuid.clone(),
            name,
        };
        let handler = get_handler().await;

        let request = handler
            .put_item()
            .table_name(&table_name)
            .item("id", AttributeValue::S(user_dto.id.to_string()))
            .item("name", AttributeValue::S(user_dto.name.to_string()));

        request
            .send()
            .await
            .map_err(|_| UserError::new(UserErrorType::Unknown, "failed to connect to db"))?;
        self.find_one(uuid).await
    }
}
