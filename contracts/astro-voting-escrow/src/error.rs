use cosmwasm_std::StdError;
use thiserror::Error;

/// ## Description
/// This enum describes maker contract errors!
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Lock already exists")]
    LockAlreadyExists {},

    #[error("Lock does not exist")]
    LockDoesntExist {},

    #[error("Lock time must be within the limits (week <= lock time < 2 years)")]
    LockTimeLimitsError {},

    #[error("Lock time cannot be reduced")]
    LockTimeDecreaseError {},

    #[error("The lock time has not yet expired")]
    LockHasNotExpired {},

    #[error("The lock expired. Withdraw and create new lock")]
    LockExpired {},
}
