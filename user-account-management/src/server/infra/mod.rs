use tonic::transport::{server::Router, Server};

use self::{
    mongodb::user::MongodbUserRepository, repository_impl::RepositoryImpls,
    resolvers::user::UserService,
};

use super::application::user::user_grpc_server::UserGrpcServer;

pub mod mongodb;
pub mod repository_impl;
pub mod resolvers;

static REPO: RepositoryImpls = RepositoryImpls {
    user_repo: MongodbUserRepository {},
};

pub fn run_server() -> Router {
    let greeter = UserService::new(&REPO);

    Server::builder().add_service(UserGrpcServer::new(greeter))
}
