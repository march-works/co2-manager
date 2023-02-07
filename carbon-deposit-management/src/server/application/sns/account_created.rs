use crate::server::domains::{
    entities::carbon_deposit::UserID, errors::carbon_deposit::CarbonDepositResult,
    repositories::Repositories, services::carbon_deposit::CarbonDepositController,
};

pub async fn handle<T: Repositories>(
    id: impl ToString,
    controller: &CarbonDepositController<'_, T>,
) -> CarbonDepositResult<()> {
    let id = UserID::try_from(id.to_string())?;
    controller.create_carbon_deposit(id).await
}
