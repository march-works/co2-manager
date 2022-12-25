use std::sync::Arc;

use tonic::transport::{server::Router, Server};

use resolvers::user_service::UserService;
use user::user_grpc_server::UserGrpcServer;

use super::infra::mongodb::user::MongodbUserRepository;

pub mod user {
    tonic::include_proto!("user");
}

mod dto;
mod resolvers;

pub fn run_server() -> Router {
    let greeter = UserService {
        repository: Arc::new(MongodbUserRepository {}),
    };

    Server::builder().add_service(UserGrpcServer::new(greeter))
}
