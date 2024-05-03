use cosmwasm_std::StdError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ContractError {
#[error("{0}")]
Std(#[from] StdError),
#[error("Insufficient LP tokens")]
InsufficientLPTokens {},

// Add more specific error cases as needed
}