use std::time::Duration;

use aws_config::{from_env, meta::region::RegionProviderChain};
use aws_sdk_dynamodb::{
    model::{AttributeDefinition, KeySchemaElement, KeyType, ScalarAttributeType, TableStatus},
    Client, Endpoint,
};

use crate::server::domains::repositories::Repositories;

use self::user::DynamodbUserRepository;

pub mod user;

pub async fn get_handler() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = from_env().region(region_provider).endpoint_resolver(Endpoint::immutable("http://dynamodb-local:8000").unwrap()).load().await;
    Client::new(&config)
}

pub async fn make_table(client: &Client, table: &str, key: &str) -> Result<(), anyhow::Error> {
    println!("make_table");
    let ad = AttributeDefinition::builder()
        .attribute_name(key)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(key)
        .key_type(KeyType::Hash)
        .build();

    // let pt = ProvisionedThroughput::builder()
    //     .read_capacity_units(10)
    //     .write_capacity_units(5)
    //     .build();

    match client
        .create_table()
        .table_name(table)
        .key_schema(ks)
        .attribute_definitions(ad)
        // .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }?;
    println!("created");

    wait_for_ready_table(client, table).await?;
    Ok(())
}

async fn wait_for_ready_table(client: &Client, table_name: &str) -> Result<(), anyhow::Error> {
    loop {
        if let Some(table) = client
            .describe_table()
            .table_name(table_name)
            .send()
            .await?
            .table()
        {
            println!("creating...");
            if !matches!(table.table_status, Some(TableStatus::Creating)) {
                break;
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(())
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
