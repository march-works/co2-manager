pub mod grpc;
pub mod sns;

use tonic::transport::{server::Router, Server};

use self::user::user_grpc_server::UserGrpcServer;
use super::infra::{dynamodb::user::DynamodbUserRepository, repository_impl::RepositoryImpls};
use crate::server::application::grpc::user::UserService;

pub mod user {
    tonic::include_proto!("user");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("user_descriptor");
}

static REPO: RepositoryImpls = RepositoryImpls {
    user_repo: DynamodbUserRepository {},
};

pub async fn run_server() -> Router {
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(user::FILE_DESCRIPTOR_SET)
        .build()
        .expect("failed to start server");

    let greeter = UserService::new(&REPO);

    Server::builder()
        .add_service(reflection)
        .add_service(UserGrpcServer::new(greeter))
}
