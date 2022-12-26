use super::domains::repositories::{user::MockUserRepository, Repositories};

mod user;

pub struct TestRepositories {
    user_repo: MockUserRepository,
}

impl TestRepositories {
    pub fn new(user_repo: MockUserRepository) -> Self {
        Self { user_repo }
    }
}

impl Repositories for TestRepositories {
    type UserRepo = MockUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
