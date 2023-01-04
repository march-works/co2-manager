use async_trait::async_trait;

use crate::server::domains::{
    entities::carbon_deposit::CarbonDeposit, errors::carbon_deposit::CarbonDepositResult,
};

#[mockall::automock]
#[async_trait]
pub trait CarbonDepositRepository {
    async fn find_one(&self, id: String) -> CarbonDepositResult<CarbonDeposit>;

    async fn update_one(&self, id: String, amount: f32) -> CarbonDepositResult<CarbonDeposit>;
}
