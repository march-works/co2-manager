use crate::server::domains::repositories::Repositories;

use super::dynamodb::user::DynamodbUserRepository;

#[derive(Clone, Debug, Default)]
pub struct RepositoryImpls {
    pub user_repo: DynamodbUserRepository,
}

impl Repositories for RepositoryImpls {
    type UserRepo = DynamodbUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
