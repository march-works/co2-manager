use std::ops;

use derive_getters::Getters;

use crate::server::domains::errors::carbon_deposit::{
    CarbonDepositError, CarbonDepositErrorType, CarbonDepositResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserID(String);

impl UserID {
    fn validate(id: &str) -> bool {
        !id.is_empty() && id.len() <= 64
    }
}

impl TryFrom<String> for UserID {
    type Error = CarbonDepositError;

    fn try_from(id: String) -> CarbonDepositResult<UserID> {
        if Self::validate(&id) {
            Ok(Self(id))
        } else {
            Err(CarbonDepositError::new(
                CarbonDepositErrorType::ParseFailed,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CarbonDepositAmount(f32);

impl CarbonDepositAmount {
    fn validate(amount: f32) -> bool {
        amount >= 0.0
    }
}

impl ops::Add for CarbonDepositAmount {
    type Output = CarbonDepositAmount;

    fn add(self, rhs: Self) -> Self::Output {
        // HACK: 正数＋正数のため常にvalid
        Self::try_from(self.0 + rhs.0).unwrap()
    }
}

impl ops::Sub for CarbonDepositAmount {
    type Output = CarbonDepositResult<CarbonDepositAmount>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::try_from(self.0 - rhs.0)
    }
}

impl TryFrom<f32> for CarbonDepositAmount {
    type Error = CarbonDepositError;

    fn try_from(amount: f32) -> CarbonDepositResult<CarbonDepositAmount> {
        if Self::validate(amount) {
            Ok(Self(amount))
        } else {
            Err(CarbonDepositError::new(
                CarbonDepositErrorType::ParseFailed,
                "amount cannot be negative",
            ))
        }
    }
}

impl From<&CarbonDepositAmount> for f32 {
    fn from(value: &CarbonDepositAmount) -> Self {
        value.0
    }
}

#[derive(Debug, Getters)]
pub struct CarbonDeposit {
    user_id: UserID,
    amount: CarbonDepositAmount,
}

impl CarbonDeposit {
    pub fn new(user_id: impl ToString, amount: f32) -> CarbonDepositResult<CarbonDeposit> {
        let user_id = UserID::try_from(user_id.to_string())?;
        let amount = CarbonDepositAmount::try_from(amount)?;
        Ok(CarbonDeposit { user_id, amount })
    }
}

impl PartialEq for CarbonDeposit {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for CarbonDeposit {}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(input, expected,
        case(-1f32, Err(CarbonDepositError { typ: CarbonDepositErrorType::ParseFailed, desc: "amount cannot be negative".into() })),
        case(0f32, Ok(CarbonDepositAmount(0f32))),
        case(1f32, Ok(CarbonDepositAmount(1f32))),
        case(0.5f32, Ok(CarbonDepositAmount(0.5f32))),
    )]
    fn validate_name(input: f32, expected: CarbonDepositResult<CarbonDepositAmount>) {
        assert_eq!(expected, CarbonDepositAmount::try_from(input));
    }
}
