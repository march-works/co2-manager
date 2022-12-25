use crate::server::domains::errors::user::{UserError, UserErrorType, UserResult};

pub struct UserID(pub String);

pub struct UserName(pub String);

pub struct User {
    pub id: UserID,
    pub name: UserName,
}

impl User {
    fn validate_name(name: &str) -> bool {
        name.len() <= 10
    }

    pub fn new(id: String, name: String) -> UserResult<User> {
        if Self::validate_name(&name) {
            Ok(User {
                id: UserID(id),
                name: UserName(name),
            })
        } else {
            Err(UserError::new(
                UserErrorType::ParseFailed,
                "failed to validate",
            ))
        }
    }
}
