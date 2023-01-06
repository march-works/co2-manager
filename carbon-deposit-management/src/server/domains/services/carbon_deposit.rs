use crate::server::domains::{
    entities::carbon_deposit::CarbonDeposit,
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

    pub async fn move_deposit(
        &self,
        from: String,
        to: String,
        amount: f32,
    ) -> CarbonDepositResult<()> {
        let from_deposit = self.carbon_repo.find_one(from.clone()).await?;
        if f32::from(from_deposit.amount()) < amount {
            return Err(CarbonDepositError::new(
                CarbonDepositErrorType::InsufficientAmount,
                "movable amount exceeded",
            ));
        }

        let to_deposit = self.carbon_repo.find_one(to.clone()).await?;
        // TODO: 更新前にこけた場合と更新後にこけた場合で分ける必要がある
        let _from_updated = self
            .carbon_repo
            .update_one(from.clone(), f32::from(from_deposit.amount()) - amount)
            .await?;
        let to_updated = self
            .carbon_repo
            .update_one(to, f32::from(to_deposit.amount()) + amount)
            .await;
        if let Err(e) = to_updated {
            self.carbon_repo
                .update_one(from, f32::from(from_deposit.amount()))
                .await?;
            Err(e)
        } else {
            Ok(())
        }
    }
}
