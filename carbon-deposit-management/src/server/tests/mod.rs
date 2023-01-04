use super::domains::repositories::{carbon_deposit::MockCarbonDepositRepository, Repositories};

mod user;

pub struct TestRepositories {
    carbon_repo: MockCarbonDepositRepository,
}

impl TestRepositories {
    pub fn new(carbon_repo: MockCarbonDepositRepository) -> Self {
        Self { carbon_repo }
    }
}

impl Repositories for TestRepositories {
    type CarbonRepo = MockCarbonDepositRepository;

    fn carbon_repository(&self) -> &Self::CarbonRepo {
        &self.carbon_repo
    }
}
