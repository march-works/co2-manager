use derive_getters::Getters;

use crate::server::domains::errors::carbon_deposit::{
    CarbonDepositError, CarbonDepositErrorType, CarbonDepositResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarbonDepositID(String);

impl CarbonDepositID {
    fn validate(id: &str) -> bool {
        !id.is_empty() && id.len() <= 64
    }
}

impl TryFrom<String> for CarbonDepositID {
    type Error = CarbonDepositError;

    fn try_from(id: String) -> CarbonDepositResult<CarbonDepositID> {
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

impl From<&CarbonDepositID> for String {
    fn from(value: &CarbonDepositID) -> Self {
        value.0.clone()
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct CarbonDepositAmount(f32);

impl CarbonDepositAmount {
    fn validate(amount: f32) -> bool {
        amount >= 0.0
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
                "failed to parse name",
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
    id: CarbonDepositID,
    user_id: UserID,
    amount: CarbonDepositAmount,
}

impl CarbonDeposit {
    pub fn new(
        id: impl ToString,
        user_id: impl ToString,
        amount: f32,
    ) -> CarbonDepositResult<CarbonDeposit> {
        let id = CarbonDepositID::try_from(id.to_string())?;
        let user_id = UserID::try_from(user_id.to_string())?;
        let amount = CarbonDepositAmount::try_from(amount)?;
        Ok(CarbonDeposit {
            id,
            user_id,
            amount,
        })
    }
}

impl PartialEq for CarbonDeposit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CarbonDeposit {}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(input, expected,
        case("", Err(CarbonDepositError { typ: CarbonDepositErrorType::ParseFailed, desc: "failed to parse id".into() })),
        case("a", Ok(CarbonDepositID("a".into()))),
        case("abcd-efgh-ijkl-mnop-qrst-uvwx-yzab-cdef-ghij-klmn-opqr-stuv-wxyz", Ok(CarbonDepositID("abcd-efgh-ijkl-mnop-qrst-uvwx-yzab-cdef-ghij-klmn-opqr-stuv-wxyz".into()))),
        case("abcd-efgh-ijkl-mnop-qrst-uvwx-yzab-cdef-ghij-klmn-opqr-stuv-wxyz-", Err(CarbonDepositError { typ: CarbonDepositErrorType::ParseFailed, desc: "failed to parse id".into() })),
    )]
    fn validate_id(input: &str, expected: CarbonDepositResult<CarbonDepositID>) {
        assert_eq!(expected, CarbonDepositID::try_from(input.to_string()));
    }

    #[rstest(input, expected,
        case("", Err(CarbonDepositError { typ: CarbonDepositErrorType::ParseFailed, desc: "failed to parse name".into() })),
        case("a", Ok(CarbonDepositAmount("a".into()))),
        case("James Bond", Ok(CarbonDepositAmount("James Bond".into()))),
        case("James Bonds", Err(CarbonDepositError { typ: CarbonDepositErrorType::ParseFailed, desc: "failed to parse name".into() })),
    )]
    fn validate_name(input: &str, expected: CarbonDepositResult<CarbonDepositAmount>) {
        assert_eq!(expected, CarbonDepositAmount::try_from(input.to_string()));
    }
}
