use std::env;

use crate::server::domains::errors::user::UserError;

use super::{get_handler, Arns};

pub async fn publish(account: &str) -> Result<(), anyhow::Error> {
    let handler = get_handler().await;
    let arns = env::var("COPILOT_SNS_TOPIC_ARNS").map_err(|_| {
        UserError::new(
            crate::server::domains::errors::user::UserErrorType::ParseFailed,
            "failed to parse env",
        )
    })?;
    println!("{arns:?}");
    let arns: Arns = serde_json::from_str(&arns)?;

    handler
        .publish()
        .topic_arn(arns.accountCreatedTopic)
        .message(account)
        .send()
        .await?;
    Ok(())
}
