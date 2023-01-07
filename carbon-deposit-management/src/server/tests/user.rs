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
        .returning(|_| CarbonDeposit::new("user-id", 100.0));

    let repositories = TestRepositories::new(user_repo);
    let controller = CarbonDepositController::new(&repositories);

    let found = controller.find_deposit("deposit-id".into()).await;
    assert!(found.is_ok());
    assert_eq!(found.unwrap(), CarbonDeposit::new("user-id", 50.0).unwrap());
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
async fn move_valid_deposit() {
    let mut user_repo = MockCarbonDepositRepository::new();
    user_repo
        .expect_find_one()
        .returning(|_| CarbonDeposit::new("user-id", 100.0));
    user_repo
        .expect_update_one()
        .returning(|id, _| CarbonDeposit::new(id, 100.0));

    let repositories = TestRepositories::new(user_repo);
    let controller = CarbonDepositController::new(&repositories);

    let move_result = controller
        .move_deposit("from-id".to_string(), "to-id".to_string(), 100.0)
        .await;
    assert!(move_result.is_ok());
}

#[tokio::test]
async fn move_exceeding_deposit() {
    let mut user_repo = MockCarbonDepositRepository::new();
    user_repo
        .expect_find_one()
        .returning(|_| CarbonDeposit::new("user-id", 100.0));
    user_repo.expect_update_one().returning(|_, _| {
        Err(CarbonDepositError::new(
            CarbonDepositErrorType::InsufficientAmount,
            "movable amount exceeded",
        ))
    });

    let repositories = TestRepositories::new(user_repo);
    let controller = CarbonDepositController::new(&repositories);

    let move_result = controller
        .move_deposit("from-id".to_string(), "to-id".to_string(), 100.0)
        .await;
    assert!(move_result.is_err());
    assert_eq!(
        move_result,
        Err(CarbonDepositError::new(
            CarbonDepositErrorType::InsufficientAmount,
            "movable amount exceeded",
        ))
    );
}
