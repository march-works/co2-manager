use std::{env, time::Duration};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client;
use serde::Deserialize;
use tokio::{runtime::Runtime, time::sleep};

use crate::server::{
    application::sns::account_created::handle,
    domains::{
        errors::carbon_deposit::{CarbonDepositError, CarbonDepositErrorType},
        repositories::Repositories,
        services::carbon_deposit::CarbonDepositController,
    },
};

use super::REPO;

pub mod account_created;

pub async fn get_handler() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Message {
    TopicArn: String,
    Message: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Arns {
    accountCreatedTopic: String,
}

async fn receive_and_handle<R: Repositories>(
    controller: &CarbonDepositController<'_, R>,
    client: &Client,
    arns: &Arns,
    url: &str,
) {
    let resp = client.receive_message().queue_url(url).send().await;
    if let Err(e) = resp {
        println!("failed to receive message: {e:?}");
        return;
    }
    if let Some(messages) = resp.unwrap().messages() {
        if !messages.is_empty() {
            for res in messages {
                if let Some(body) = res.body() {
                    let body = serde_json::from_str::<Message>(body);
                    match body {
                        Ok(message) => {
                            // topicを増やした場合はhandlerを追加する
                            if message.TopicArn == arns.accountCreatedTopic {
                                let handled = handle(message.Message, controller).await;
                                if let Err(e) = handled {
                                    println!("topic handle error: {e:?}");
                                }
                            } else {
                                println!("no topic matched: {}", message.TopicArn);
                            }
                        }
                        Err(e) => {
                            println!("topic body parse failed: {e:?}");
                        }
                    }
                }
            }
            if let Some(handle) = messages[0].receipt_handle() {
                let sent = client
                    .delete_message()
                    .receipt_handle(handle)
                    .queue_url(url)
                    .send()
                    .await;
                if let Err(e) = sent {
                    println!("topic delete error: {e:?}");
                }
            }
        }
    }
}

pub fn subscribe() -> Result<(), anyhow::Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let controller = CarbonDepositController::new(&REPO);
        let client = get_handler().await;
        let url = env::var("COPILOT_QUEUE_URI").map_err(|_| {
            CarbonDepositError::new(CarbonDepositErrorType::Unknown, "failed to parse env")
        })?;
        let arns = env::var("COPILOT_SNS_TOPIC_ARNS").map_err(|_| {
            CarbonDepositError::new(
                crate::server::domains::errors::carbon_deposit::CarbonDepositErrorType::ParseFailed,
                "failed to parse env",
            )
        })?;
        println!("{arns}");
        let arns: Arns = serde_json::from_str(&arns)?;

        loop {
            receive_and_handle(&controller, &client, &arns, &url).await;
            sleep(Duration::from_secs(10)).await;
        }
    })
}
