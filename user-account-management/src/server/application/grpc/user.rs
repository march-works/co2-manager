use tonic::{Request, Response, Status};

use crate::server::{
    application::user::{
        user_grpc_server::UserGrpc, CreateUserRequest, CreateUserResponse, GetUserRequest,
        GetUserResponse,
    },
    domains::{
        errors::user::{UserError, UserErrorType},
        services::user::UserController,
    },
    infra::{repository_impl::RepositoryImpls, sns::account_created},
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
            Ok(user) => {
                let res = account_created::publish(&String::from(user.id())).await;
                if res.is_ok() {
                    Ok(Response::new(CreateUserResponse {
                        id: user.id().into(),
                        name: user.name().into(),
                    }))
                } else {
                    // TODO: ユーザー作成のロールバック処理
                    Err(Status::unavailable(format!(
                        "message service is temporary unavailable: {:?}",
                        res.err().unwrap()
                    )))
                }
            }
            Err(UserError {
                typ: UserErrorType::Duplicate,
                desc,
            }) => Err(Status::invalid_argument(format!("duplicate id: {desc}"))),
            Err(UserError {
                typ: UserErrorType::ParseFailed,
                desc,
            }) => Err(Status::invalid_argument(format!(
                "invalid id format: {desc}"
            ))),
            Err(e) => Err(Status::internal(e.desc)),
        }
    }
}
