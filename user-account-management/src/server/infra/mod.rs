use tonic::transport::{server::Router, Server};

use self::{
    dynamodb::{user::DynamodbUserRepository, make_table, get_handler}, repository_impl::RepositoryImpls,
    resolvers::user::UserService,
};

use super::application::user::user_grpc_server::UserGrpcServer;

pub mod dynamodb;
pub mod repository_impl;
pub mod resolvers;

mod proto {
    tonic::include_proto!("user");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("user_descriptor");
}

static REPO: RepositoryImpls = RepositoryImpls {
    user_repo: DynamodbUserRepository {},
};

pub async fn run_server() -> Router {
    let ret = make_table(&get_handler().await, "users", "id").await;
    if let Err(e) = ret {
        println!("{:?}", e);
    } else {
        println!("table successfully created!!");
    }

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let greeter = UserService::new(&REPO);

    Server::builder()
        .add_service(reflection)
        .add_service(UserGrpcServer::new(greeter))
}
