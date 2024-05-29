use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Program not found")]
    ProgramNotFound {},
    #[error("Insufficient staked amount")]
    InsufficientStakedAmount {},
    #[error("Invalid CW20 receive message")]
    InvalidCw20ReceiveMsg {},
    #[error("Invalid reward token")]
    InvalidRewardToken {},
    #[error("Insufficient reward pool balance")]
    InsufficientRewardPoolBalance {},
}

impl From<cw20_base::ContractError> for ContractError {
    fn from(err: cw20_base::ContractError) -> Self {
        ContractError::Std(StdError::generic_err(format!("{}", err)))
    }
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> StdError {
        match err {
            ContractError::Std(e) => e,
            ContractError::Unauthorized { .. } => StdError::generic_err("Unauthorized"),
            ContractError::ProgramNotFound { .. } => StdError::generic_err("Program not found"),
            ContractError::InsufficientStakedAmount { .. } => StdError::generic_err("Insufficient staked amount"),
            ContractError::InvalidCw20ReceiveMsg { .. } => StdError::generic_err("Invalid CW20 receive message"),
            ContractError::InvalidRewardToken { .. } => StdError::generic_err("Invalid reward token"),
            ContractError::InsufficientRewardPoolBalance { .. } => StdError::generic_err("Insufficient balance in the CNSTE reward pool"),
        }
    }
}