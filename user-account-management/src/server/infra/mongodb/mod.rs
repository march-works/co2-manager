use std::env;

use mongodb::{options::ClientOptions, Client, Database};

use crate::server::domains::repositories::Repositories;

use self::user::MongodbUserRepository;

pub mod user;

async fn get_handler() -> Result<Database, anyhow::Error> {
    let username = env::var("MONGO_USERNAME")?;
    let password = env::var("MONGO_PASSWORD")?;
    let url = env::var("MONGO_ACCESS_URL")?;
    let conf = env::var("MONGO_ACCESS_CONF")?;
    let mut client_options = ClientOptions::parse(format!(
        "mongodb://{}:{}@{}:27017{}",
        username, password, url, conf
    ))
    .await?;
    client_options.app_name = Some("User Account Management".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client.database("user-account"))
}

struct MongodbRepositories {
    user_repo: MongodbUserRepository,
}

impl Repositories for MongodbRepositories {
    type UserRepo = MongodbUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
