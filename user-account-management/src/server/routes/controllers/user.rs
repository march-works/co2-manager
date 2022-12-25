use crate::server::domains::{
    entities::user::User,
    errors::user::UserResult,
    repositories::{user::UserRepository, Repositories},
};

pub struct UserController<'r, R: Repositories> {
    user_repo: &'r R::UserRepo,
}

impl<'r, R: Repositories> UserController<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            user_repo: repositories.user_repository(),
        }
    }

    pub async fn find_user(&self, id: String) -> UserResult<User> {
        self.user_repo.find_one(id).await
    }

    pub async fn create_user(&self, name: String) -> UserResult<User> {
        self.user_repo.create_one(name).await
    }
}
