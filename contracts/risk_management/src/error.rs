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
}

