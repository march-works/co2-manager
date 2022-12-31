use std::env;

use aws_config::{from_env, meta::region::RegionProviderChain};
use aws_sdk_dynamodb::{Client, Endpoint};

use crate::server::domains::repositories::Repositories;

use self::user::DynamodbUserRepository;

pub mod user;

pub async fn get_handler() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let mut config = from_env().region(region_provider);
    if use_localstack() {
        config = config.endpoint_resolver(localstack_endpoint());
    }
    let config = config.load().await;
    Client::new(&config)
}

fn use_localstack() -> bool {
    env::var("LOCALSTACK").unwrap_or_default() == "true"
}

fn localstack_endpoint() -> Endpoint {
    Endpoint::immutable("http://dynamodb-local:8000/").expect("valid endpoint")
}

struct DynamodbRepositories {
    user_repo: DynamodbUserRepository,
}

impl Repositories for DynamodbRepositories {
    type UserRepo = DynamodbUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
