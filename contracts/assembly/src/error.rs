use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

/// This enum describes Assembly contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Proposal not active!")]
    ProposalNotActive {},

    #[error("Voting period ended!")]
    VotingPeriodEnded {},

    #[error("User already voted!")]
    UserAlreadyVoted {},

    #[error("You don't have any voting power!")]
    NoVotingPower {},

    #[error("Voting period not ended yet!")]
    VotingPeriodNotEnded {},

    #[error("Proposal expired!")]
    ExecuteProposalExpired {},

    #[error("Insufficient token deposit!")]
    InsufficientDeposit {},

    #[error("Proposal not passed!")]
    ProposalNotPassed {},

    #[error("Proposal not completed!")]
    ProposalNotCompleted {},

    #[error("Proposal delay not ended!")]
    ProposalDelayNotEnded {},

    #[error("Contract can't be migrated!")]
    MigrationError {},

    #[error("Whitelist cannot be empty!")]
    WhitelistEmpty {},

    #[error("Messages check passed. Nothing was committed to the blockchain")]
    MessagesCheckPassed {},
}

impl From<OverflowError> for ContractError {
    fn from(o: OverflowError) -> Self {
        StdError::from(o).into()
    }
}
