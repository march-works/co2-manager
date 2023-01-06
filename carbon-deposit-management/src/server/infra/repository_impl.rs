use crate::server::domains::repositories::Repositories;

use super::dynamodb::carbon_deposit::DynamodbCarbonDepositRepository;

#[derive(Clone, Debug, Default)]
pub struct RepositoryImpls {
    pub carbon_repo: DynamodbCarbonDepositRepository,
}

impl Repositories for RepositoryImpls {
    type CarbonRepo = DynamodbCarbonDepositRepository;

    fn carbon_repository(&self) -> &Self::CarbonRepo {
        &self.carbon_repo
    }
}
