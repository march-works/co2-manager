use derive_getters::Getters;

use crate::server::domains::errors::user::{UserError, UserErrorType, UserResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserID(String);

impl UserID {
    fn validate(id: &str) -> bool {
        id.is_empty() && id.len() <= 64
    }
}

impl TryFrom<String> for UserID {
    type Error = UserError;

    fn try_from(id: String) -> UserResult<UserID> {
        if Self::validate(&id) {
            Ok(Self(id))
        } else {
            Err(UserError::new(
                UserErrorType::ParseFailed,
                "failed to parse id",
            ))
        }
    }
}

impl From<&UserID> for String {
    fn from(value: &UserID) -> Self {
        value.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UserName(String);

impl UserName {
    fn validate(name: &str) -> bool {
        name.is_empty() && name.len() <= 10
    }
}

impl TryFrom<String> for UserName {
    type Error = UserError;

    fn try_from(name: String) -> UserResult<UserName> {
        if Self::validate(&name) {
            Ok(Self(name))
        } else {
            Err(UserError::new(
                UserErrorType::ParseFailed,
                "failed to parse id",
            ))
        }
    }
}

impl From<&UserName> for String {
    fn from(value: &UserName) -> Self {
        value.0.clone()
    }
}

#[derive(Debug, Getters)]
pub struct User {
    id: UserID,
    name: UserName,
}

impl User {
    pub fn new(id: impl ToString, name: impl ToString) -> UserResult<User> {
        let id = UserID::try_from(id.to_string())?;
        let name = UserName::try_from(name.to_string())?;
        Ok(User { id, name })
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}
