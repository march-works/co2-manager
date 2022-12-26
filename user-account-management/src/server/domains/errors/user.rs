use std::{error::Error, fmt::Display};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum UserErrorType {
    NotFound,
    Duplicate,
    ParseFailed,
    Unknown,
}

pub type UserResult<T> = Result<T, UserError>;

#[derive(Debug, PartialEq, Eq)]
pub struct UserError {
    pub typ: UserErrorType,
    pub desc: String,
}

impl UserError {
    pub fn new(typ: UserErrorType, desc: impl ToString) -> Self {
        Self {
            typ,
            desc: desc.to_string(),
        }
    }
}

impl Error for UserError {}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.typ, self.desc)
    }
}
