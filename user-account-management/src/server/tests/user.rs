use crate::server::{
    application::controllers::user::UserController,
    domains::{
        entities::user::User,
        errors::user::{UserError, UserErrorType},
        repositories::user::MockUserRepository,
    },
};

use super::TestRepositories;

#[tokio::test]
async fn find_existing_user() {
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_find_one()
        .returning(|_| User::new("id-test", "James Bond"));

    let repositories = TestRepositories::new(user_repo);
    let controller = UserController::new(&repositories);

    let found = controller.find_user("id-test".into()).await;
    assert!(found.is_ok());
    assert_eq!(found.unwrap(), User::new("id-test", "James").unwrap());
}

#[tokio::test]
async fn find_not_existing_user() {
    let mut user_repo = MockUserRepository::new();
    user_repo.expect_find_one().returning(|_| {
        Err(UserError::new(
            UserErrorType::NotFound,
            "user not found for given id",
        ))
    });

    let repositories = TestRepositories::new(user_repo);
    let controller = UserController::new(&repositories);

    let found = controller.find_user("id-test".into()).await;
    assert!(found.is_err());
    assert_eq!(
        found.unwrap_err(),
        UserError::new(UserErrorType::NotFound, "user not found for given id")
    );
}

#[tokio::test]
async fn create_new_user() {
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_create_one()
        .returning(|_| User::new("id-test", "James Bond"));

    let repositories = TestRepositories::new(user_repo);
    let controller = UserController::new(&repositories);

    let created = controller.create_user("James Bond".into()).await;
    assert!(created.is_ok());
    assert_eq!(
        created.unwrap(),
        User::new("id-test", "James Bond").unwrap()
    );
}

#[tokio::test]
async fn create_duplicate_user() {
    let mut user_repo = MockUserRepository::new();
    user_repo.expect_create_one().returning(|_| {
        Err(UserError::new(
            UserErrorType::Duplicate,
            "user already exists for given id",
        ))
    });

    let repositories = TestRepositories::new(user_repo);
    let controller = UserController::new(&repositories);

    let created = controller.create_user("James Bond".into()).await;
    assert!(created.is_err());
    assert_eq!(
        created.unwrap_err(),
        UserError::new(UserErrorType::Duplicate, "user already exists for given id")
    );
}
