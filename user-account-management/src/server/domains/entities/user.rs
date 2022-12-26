use derive_getters::Getters;

use crate::server::domains::errors::user::{UserError, UserErrorType, UserResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserID(String);

impl UserID {
    fn validate(id: &str) -> bool {
        !id.is_empty() && id.len() <= 64
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    fn validate(name: &str) -> bool {
        !name.is_empty() && name.len() <= 10
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
                "failed to parse name",
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(input, expected,
        case("", Err(UserError { typ: UserErrorType::ParseFailed, desc: "failed to parse id".into() })),
        case("a", Ok(UserID("a".into()))),
        case("abcd-efgh-ijkl-mnop-qrst-uvwx-yzab-cdef-ghij-klmn-opqr-stuv-wxyz", Ok(UserID("abcd-efgh-ijkl-mnop-qrst-uvwx-yzab-cdef-ghij-klmn-opqr-stuv-wxyz".into()))),
        case("abcd-efgh-ijkl-mnop-qrst-uvwx-yzab-cdef-ghij-klmn-opqr-stuv-wxyz-", Err(UserError { typ: UserErrorType::ParseFailed, desc: "failed to parse id".into() })),
    )]
    fn validate_id(input: &str, expected: UserResult<UserID>) {
        assert_eq!(expected, UserID::try_from(input.to_string()));
    }

    #[rstest(input, expected,
        case("", Err(UserError { typ: UserErrorType::ParseFailed, desc: "failed to parse name".into() })),
        case("a", Ok(UserName("a".into()))),
        case("James Bond", Ok(UserName("James Bond".into()))),
        case("James Bonds", Err(UserError { typ: UserErrorType::ParseFailed, desc: "failed to parse name".into() })),
    )]
    fn validate_name(input: &str, expected: UserResult<UserName>) {
        assert_eq!(expected, UserName::try_from(input.to_string()));
    }
}
