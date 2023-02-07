use crate::server::domains::{
    entities::carbon_deposit::{CarbonDeposit, CarbonDepositAmount, UserID},
    errors::carbon_deposit::{CarbonDepositError, CarbonDepositErrorType, CarbonDepositResult},
    repositories::{carbon_deposit::CarbonDepositRepository, Repositories},
};

pub struct CarbonDepositController<'r, R: Repositories> {
    carbon_repo: &'r R::CarbonRepo,
}

impl<'r, R: Repositories> CarbonDepositController<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            carbon_repo: repositories.carbon_repository(),
        }
    }

    pub async fn find_deposit(&self, id: String) -> CarbonDepositResult<CarbonDeposit> {
        self.carbon_repo.find_one(id).await
    }

    pub async fn create_carbon_deposit(&self, id: UserID) -> CarbonDepositResult<()> {
        self.carbon_repo.create_one(String::from(&id)).await?;
        Ok(())
    }

    pub async fn move_deposit(
        &self,
        from: UserID,
        to: UserID,
        amount: CarbonDepositAmount,
    ) -> CarbonDepositResult<()> {
        let from_deposit = self.carbon_repo.find_one(String::from(&from)).await?;
        let to_deposit = self.carbon_repo.find_one(String::from(&to)).await?;
        let post_from_amount = (from_deposit.amount().clone() - amount.clone()).map_err(|_| {
            CarbonDepositError::new(
                CarbonDepositErrorType::InsufficientAmount,
                "movable amount exceeded",
            )
        })?;
        let post_to_amount = to_deposit.amount().clone() + amount.clone();

        // TODO: 更新前にこけた場合と更新後にこけた場合で分ける必要がある
        let _from_updated = self
            .carbon_repo
            .update_one(String::from(&from), f32::from(&post_from_amount))
            .await?;
        let to_updated = self
            .carbon_repo
            .update_one(String::from(&to), f32::from(&post_to_amount))
            .await;
        if let Err(e) = to_updated {
            self.carbon_repo
                .update_one(String::from(&from), f32::from(from_deposit.amount()))
                .await?;
            Err(e)
        } else {
            Ok(())
        }
    }
}
