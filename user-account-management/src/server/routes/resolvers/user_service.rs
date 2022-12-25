use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::server::{
    domains::repositories::user::UserRepository,
    routes::user::{CreateUserRequest, CreateUserResponse},
};

use super::super::user::{self, user_grpc_server::UserGrpc, GetUserRequest, GetUserResponse};

pub struct UserService<T: UserRepository + Send + Sync> {
    pub repository: Arc<T>,
}

#[tonic::async_trait]
impl<T: UserRepository + Send + Sync + 'static> UserGrpc for UserService<T> {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let reply = user::GetUserResponse {
            name: format!("Hello {}!", request.into_inner().id).into(),
        };

        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let created = self.repository.create(request.into_inner().name).await;
        match created {
            Ok(user) => Ok(Response::new(CreateUserResponse {
                id: user.id.0,
                name: user.name.0,
            })),
            Err(_) => Err(Status::invalid_argument("failed to create")),
        }
    }
}
