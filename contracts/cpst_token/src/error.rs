use cosmwasm_std::StdError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Insufficient balance")]
    InsufficientBalance {},

    #[error("Invalid Execute Msg")]
    InvalidExecuteMsg {},

    #[error("Unsupported Execute Msg")]
    UnsupportedExecuteMsg {},
}