use tonic::{Request, Response, Status};

use super::super::user::{self, user_server::User, UserRequest, UserResponse};

#[derive(Debug, Default)]
pub struct UserService {}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // println!("Got a request: {:?}", request);

        let reply = user::UserResponse {
            name: format!("Good night {}!", request.into_inner().id).into(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
