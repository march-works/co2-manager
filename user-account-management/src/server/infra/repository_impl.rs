use crate::server::domains::repositories::Repositories;

use super::mongodb::user::MongodbUserRepository;

#[derive(Clone, Debug, Default)]
pub struct RepositoryImpls {
    pub user_repo: MongodbUserRepository,
}

impl Repositories for RepositoryImpls {
    type UserRepo = MongodbUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
