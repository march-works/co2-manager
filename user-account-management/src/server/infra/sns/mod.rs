use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sns::Client;
use serde::Deserialize;

pub mod account_created;

pub async fn get_handler() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Arns {
    accountCreatedTopic: String,
}
