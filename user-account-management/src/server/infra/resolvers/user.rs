use tonic::{Request, Response, Status};

use crate::server::{
    application::{
        controllers::user::UserController,
        user::{
            user_grpc_server::UserGrpc, CreateUserRequest, CreateUserResponse, GetUserRequest,
            GetUserResponse,
        },
    },
    domains::errors::user::{UserError, UserErrorType},
    infra::repository_impl::RepositoryImpls,
};

pub struct UserService<'r> {
    controller: UserController<'r, RepositoryImpls>,
}

impl<'r> UserService<'r> {
    pub fn new(repositories: &'r RepositoryImpls) -> Self {
        let controller = UserController::new(repositories);
        Self { controller }
    }
}

#[tonic::async_trait]
impl UserGrpc for UserService<'static> {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let found = self.controller.find_user(request.into_inner().id).await;
        match found {
            Ok(user) => Ok(Response::new(GetUserResponse {
                name: user.name().into(),
            })),
            Err(UserError {
                typ: UserErrorType::NotFound,
                desc,
            }) => Err(Status::not_found(desc)),
            Err(UserError {
                typ: UserErrorType::ParseFailed,
                desc,
            }) => Err(Status::invalid_argument(desc)),
            Err(UserError { typ: _, desc }) => Err(Status::unknown(desc)),
        }
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let created = self.controller.create_user(request.into_inner().name).await;
        match created {
            Ok(user) => Ok(Response::new(CreateUserResponse {
                id: user.id().into(),
                name: user.name().into(),
            })),
            Err(_) => Err(Status::invalid_argument("failed to create")),
        }
    }
}
