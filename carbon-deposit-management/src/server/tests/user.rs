use crate::server::{
    domains::services::carbon_deposit::CarbonDepositController,
    domains::{
        entities::carbon_deposit::CarbonDeposit,
        errors::carbon_deposit::{CarbonDepositError, CarbonDepositErrorType},
        repositories::carbon_deposit::MockCarbonDepositRepository,
    },
};

use super::TestRepositories;

#[tokio::test]
async fn find_existing_user() {
    let mut user_repo = MockCarbonDepositRepository::new();
    user_repo
        .expect_find_one()
        .returning(|_| CarbonDeposit::new("deposit-id", "user-id", 100.0));

    let repositories = TestRepositories::new(user_repo);
    let controller = CarbonDepositController::new(&repositories);

    let found = controller.find_deposit("deposit-id".into()).await;
    assert!(found.is_ok());
    assert_eq!(
        found.unwrap(),
        CarbonDeposit::new("deposit-id", "another-user-id", 50.0).unwrap()
    );
}

#[tokio::test]
async fn find_not_existing_user() {
    let mut user_repo = MockCarbonDepositRepository::new();
    user_repo.expect_find_one().returning(|_| {
        Err(CarbonDepositError::new(
            CarbonDepositErrorType::NotFound,
            "user not found for given id",
        ))
    });

    let repositories = TestRepositories::new(user_repo);
    let controller = CarbonDepositController::new(&repositories);

    let found = controller.find_deposit("deposit-id".into()).await;
    assert!(found.is_err());
    assert_eq!(
        found.unwrap_err(),
        CarbonDepositError::new(
            CarbonDepositErrorType::NotFound,
            "user not found for given id"
        )
    );
}

#[tokio::test]
async fn move_zero_deposit() {
    let mut user_repo = MockCarbonDepositRepository::new();
    user_repo
        .expect_move_deposit()
        .returning(|from, to, amount| Ok(()));

    let repositories = TestRepositories::new(user_repo);
    let controller = CarbonDepositController::new(&repositories);

    let created = controller
        .move_deposit("from-id".to_string(), "to-id".to_string(), 100.0)
        .await;
    assert!(created.is_ok());
    assert_eq!(
        created.unwrap(),
        CarbonDeposit::new("id-test", "James Bond").unwrap()
    );
}
