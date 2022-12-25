use tonic::transport::{server::Router, Server};

use resolvers::user_service::UserService;
use user::user_server::UserServer;

pub mod user {
    tonic::include_proto!("user");
}

mod dto;
mod resolvers;

pub fn run_server() -> Router {
    let greeter = UserService::default();

    Server::builder().add_service(UserServer::new(greeter))
}
