pub mod grpc;
pub mod sns;

pub mod carbon_deposit {
    tonic::include_proto!("carbon_deposit");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("carbon_deposit_descriptor");
}

use std::thread;

use tonic::transport::{server::Router, Server};

use self::carbon_deposit::carbon_deposit_grpc_server::CarbonDepositGrpcServer;
use self::grpc::carbon_deposit::CarbonDepositService;
use self::sns::subscribe;
use crate::server::infra::{
    dynamodb::carbon_deposit::DynamodbCarbonDepositRepository, repository_impl::RepositoryImpls,
};

static REPO: RepositoryImpls = RepositoryImpls {
    carbon_repo: DynamodbCarbonDepositRepository {},
};

pub fn subscribe_queues() {
    thread::spawn(subscribe);
}

pub async fn run_server() -> Router {
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(carbon_deposit::FILE_DESCRIPTOR_SET)
        .build()
        .expect("failed to start server");

    let greeter = CarbonDepositService::new(&REPO);

    Server::builder()
        .add_service(reflection)
        .add_service(CarbonDepositGrpcServer::new(greeter))
}
