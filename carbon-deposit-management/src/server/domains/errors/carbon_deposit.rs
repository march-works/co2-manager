use std::{error::Error, fmt::Display};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CarbonDepositErrorType {
    NotFound,
    Duplicate,
    ParseFailed,
    InsufficientAmount,
    Unknown,
}

pub type CarbonDepositResult<T> = Result<T, CarbonDepositError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CarbonDepositError {
    pub typ: CarbonDepositErrorType,
    pub desc: String,
}

impl CarbonDepositError {
    pub fn new(typ: CarbonDepositErrorType, desc: impl ToString) -> Self {
        Self {
            typ,
            desc: desc.to_string(),
        }
    }
}

impl Error for CarbonDepositError {}

impl Display for CarbonDepositError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.typ, self.desc)
    }
}
