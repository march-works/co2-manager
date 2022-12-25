use std::env;

use async_trait::async_trait;
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::domains::{
    entities::user::User,
    repositories::user::{UserCreateFailed, UserRepository},
};

#[derive(Debug, Serialize, Deserialize)]
struct UserDto {
    id: String,
    name: String,
}

impl UserDto {
    fn from(user_dto: User) -> Self {
        UserDto {
            id: user_dto.id.0,
            name: user_dto.name.0,
        }
    }

    fn into(self) -> Option<User> {
        User::new(self.id, self.name)
    }
}

pub struct MongodbUserRepository;

impl MongodbUserRepository {
    async fn get_handler(&self) -> Result<Client, anyhow::Error> {
        let username = env::var("MONGO_USERNAME")?;
        let password = env::var("MONGO_PASSWORD")?;
        println!("{}", username);
        println!("{}", password);
        let mut client_options = ClientOptions::parse(format!(
            "mongodb://{}:{}@mongo-user:27017",
            username, password
        ))
        .await?;
        client_options.app_name = Some("User Account Management".to_string());
        Ok(Client::with_options(client_options)?)
    }
}

#[async_trait]
impl UserRepository for MongodbUserRepository {
    async fn create(&self, name: String) -> Result<User, UserCreateFailed> {
        let uuid = Uuid::new_v4().to_string();
        let user_dto = UserDto {
            id: uuid.clone(),
            name,
        };
        let handler = self.get_handler().await;
        let ret = match handler {
            Ok(client) => {
                let db = client.database("user-account");
                let collection = db.collection::<UserDto>("users");
                collection
                    .insert_one(&user_dto, None)
                    .await
                    .or(Err(UserCreateFailed))?;

                let filter = doc! { "id": uuid };
                let cursor = collection
                    .find(filter, None)
                    .await
                    .or(Err(UserCreateFailed))?;
                let created = cursor.deserialize_current().or(Err(UserCreateFailed))?;
                created.into()
            }
            Err(_) => None,
        };
        ret.ok_or(UserCreateFailed)
    }
}
