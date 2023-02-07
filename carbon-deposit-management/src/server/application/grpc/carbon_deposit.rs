use tonic::{Request, Response, Status};

use crate::server::application::carbon_deposit::carbon_deposit_grpc_server::CarbonDepositGrpc;
use crate::server::application::carbon_deposit::{
    GetUserCarbonDepositRequest, GetUserCarbonDepositResponse,
};
use crate::server::application::carbon_deposit::{
    MoveDepositBetweenUserRequest, MoveDepositBetweenUserResponse,
};
use crate::server::domains::entities::carbon_deposit::{CarbonDepositAmount, UserID};
use crate::server::{
    domains::errors::carbon_deposit::{CarbonDepositError, CarbonDepositErrorType},
    domains::services::carbon_deposit::CarbonDepositController,
    infra::repository_impl::RepositoryImpls,
};

pub struct CarbonDepositService<'r> {
    controller: CarbonDepositController<'r, RepositoryImpls>,
}

impl<'r> CarbonDepositService<'r> {
    pub fn new(repositories: &'r RepositoryImpls) -> Self {
        let controller = CarbonDepositController::new(repositories);
        Self { controller }
    }
}

#[tonic::async_trait]
impl CarbonDepositGrpc for CarbonDepositService<'static> {
    async fn get_user_carbon_deposit(
        &self,
        request: Request<GetUserCarbonDepositRequest>,
    ) -> Result<Response<GetUserCarbonDepositResponse>, Status> {
        let found = self
            .controller
            .find_deposit(request.into_inner().user_id)
            .await;
        match found {
            Ok(deposit) => Ok(Response::new(GetUserCarbonDepositResponse {
                user_id: deposit.user_id().into(),
                amount: deposit.amount().into(),
            })),
            Err(CarbonDepositError {
                typ: CarbonDepositErrorType::NotFound,
                desc,
            }) => Err(Status::not_found(desc)),
            Err(CarbonDepositError {
                typ: CarbonDepositErrorType::ParseFailed,
                desc,
            }) => Err(Status::invalid_argument(desc)),
            Err(CarbonDepositError { typ: _, desc }) => Err(Status::unknown(desc)),
        }
    }

    async fn move_deposit_between_user(
        &self,
        request: Request<MoveDepositBetweenUserRequest>,
    ) -> Result<Response<MoveDepositBetweenUserResponse>, Status> {
        let MoveDepositBetweenUserRequest { from, to, amount } = request.into_inner();
        let from = UserID::try_from(from)
            .map_err(|e| Status::internal(format!("invalid amount data: {}", e.desc)))?;
        let to = UserID::try_from(to)
            .map_err(|e| Status::internal(format!("invalid amount data: {}", e.desc)))?;
        let amount = CarbonDepositAmount::try_from(amount)
            .map_err(|e| Status::internal(format!("invalid amount data: {}", e.desc)))?;
        let result = self.controller.move_deposit(from, to, amount).await;
        match result {
            Ok(_) => Ok(Response::new(MoveDepositBetweenUserResponse {
                moved_result: "Yeah!".to_string(),
            })),
            Err(CarbonDepositError {
                typ: CarbonDepositErrorType::InsufficientAmount,
                desc,
            }) => Err(Status::invalid_argument(desc)),
            Err(CarbonDepositError {
                typ: CarbonDepositErrorType::ParseFailed,
                desc,
            }) => Err(Status::internal(format!("invalid amount data: {desc}"))),
            Err(e) => Err(Status::internal(e.desc)),
        }
    }
}
