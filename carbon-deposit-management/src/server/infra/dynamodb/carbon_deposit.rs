use std::{
    collections::HashMap,
    env,
    fmt::{Debug, Display},
};

use async_trait::async_trait;
use aws_sdk_dynamodb::model::{AttributeAction, AttributeValue, AttributeValueUpdate, ReturnValue};

use crate::server::domains::{
    entities::carbon_deposit::CarbonDeposit,
    errors::carbon_deposit::{CarbonDepositError, CarbonDepositErrorType, CarbonDepositResult},
    repositories::carbon_deposit::CarbonDepositRepository,
};

use super::get_handler;

#[derive(Clone, Debug, Default)]
pub struct DynamodbCarbonDepositRepository;

impl DynamodbCarbonDepositRepository {
    fn not_found(e: impl Display) -> CarbonDepositError {
        CarbonDepositError::new(CarbonDepositErrorType::NotFound, format!("{e}"))
    }

    fn parse_failed(field: &str) -> CarbonDepositError {
        CarbonDepositError::new(
            CarbonDepositErrorType::ParseFailed,
            format!("could not parse {field}"),
        )
    }

    fn retrieve(
        attributes: &HashMap<String, AttributeValue>,
    ) -> CarbonDepositResult<CarbonDeposit> {
        let user_id = attributes
            .get("userId")
            .ok_or_else(|| Self::parse_failed("user_id"))?
            .as_s()
            .map_err(|_| Self::parse_failed("user_id"))?;
        let amount = attributes
            .get("amount")
            .ok_or_else(|| Self::parse_failed("amount"))?
            .as_n()
            .map(|v| v.parse::<f32>())
            .map_err(|_| Self::parse_failed("amount"))?
            .map_err(|e| {
                CarbonDepositError::new(CarbonDepositErrorType::Unknown, format!("{e}"))
            })?;
        CarbonDeposit::new(user_id, amount)
    }
}

#[async_trait]
impl CarbonDepositRepository for DynamodbCarbonDepositRepository {
    async fn create_one(&self, id: String) -> CarbonDepositResult<CarbonDeposit> {
        let table_name = env::var("CARBONDEPOSITS_NAME").map_err(|_| {
            CarbonDepositError::new(CarbonDepositErrorType::Unknown, "failed to parse env")
        })?;
        let handler = get_handler().await;
        handler
            .put_item()
            .table_name(&table_name)
            .item("userId", AttributeValue::S(id.clone()))
            .item("amount", AttributeValue::N("0".to_string()))
            .send()
            .await
            .map_err(|e| {
                CarbonDepositError::new(
                    CarbonDepositErrorType::Unknown,
                    format!("failed to connect to db: {e:?}"),
                )
            })?;
        self.find_one(id).await
    }

    async fn find_one(&self, id: String) -> CarbonDepositResult<CarbonDeposit> {
        let table_name = env::var("CARBONDEPOSITS_NAME").map_err(|_| {
            CarbonDepositError::new(CarbonDepositErrorType::Unknown, "failed to parse env")
        })?;
        let handler = get_handler().await;
        let item = handler
            .get_item()
            .table_name(&table_name)
            .key("userId", AttributeValue::S(id.clone()))
            .send()
            .await
            .map_err(|e| {
                CarbonDepositError::new(
                    CarbonDepositErrorType::Unknown,
                    format!("failed to connect to db: {e:?}"),
                )
            })?;
        if let Some(deposit) = item.item() {
            Self::retrieve(deposit)
        } else {
            Err(Self::not_found(format!("not found for id: {id}")))
        }
    }

    async fn update_one(&self, id: String, amount: f32) -> CarbonDepositResult<CarbonDeposit> {
        let table_name = env::var("CARBONDEPOSITS_NAME").map_err(|_| {
            CarbonDepositError::new(CarbonDepositErrorType::Unknown, "failed to parse env")
        })?;
        let handler = get_handler().await;
        let before = AttributeValueUpdate::builder()
            .action(AttributeAction::Put)
            .value(AttributeValue::N(amount.to_string()))
            .build();
        let updated = handler
            .update_item()
            .table_name(&table_name)
            .key("userId", AttributeValue::S(id.clone()))
            .attribute_updates("amount", before)
            .return_values(ReturnValue::AllNew)
            .send()
            .await
            .map_err(|_| {
                CarbonDepositError::new(CarbonDepositErrorType::Unknown, "failed to connect to db")
            })?;

        if let Some(deposit) = updated.attributes() {
            Self::retrieve(deposit)
        } else {
            Err(Self::not_found(format!("not found for id: {id}")))
        }
    }
}
