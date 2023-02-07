use std::{env, time::Duration};

use tokio::{runtime::Runtime, time::sleep};

use crate::server::{
    application::REPO,
    domains::{
        entities::carbon_deposit::UserID,
        errors::carbon_deposit::{CarbonDepositError, CarbonDepositErrorType},
        services::carbon_deposit::CarbonDepositController,
    },
};

use super::get_handler;

pub fn subscribe() -> Result<(), anyhow::Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let controller = CarbonDepositController::new(&REPO);
        let client = get_handler().await;
        let url = env::var("COPILOT_QUEUE_URI").map_err(|_| {
            CarbonDepositError::new(CarbonDepositErrorType::Unknown, "failed to parse env")
        })?;
        loop {
            let resp = client.receive_message().queue_url(&url).send().await?;
            if let Some(messages) = resp.messages() {
                if !messages.is_empty() {
                    for res in messages {
                        println!("{res:?}");
                        if let Some(id) = res.body() {
                            if let Ok(id) = UserID::try_from(id.to_string()) {
                                controller.create_carbon_deposit(id).await?;
                            }
                        }
                    }
                    if let Some(handle) = messages[0].receipt_handle() {
                        client
                            .delete_message()
                            .receipt_handle(handle)
                            .queue_url(&url)
                            .send()
                            .await?;
                    }
                }
            }
            sleep(Duration::from_secs(10)).await;
        }
    })
}
