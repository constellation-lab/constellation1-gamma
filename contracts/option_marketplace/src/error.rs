use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Option not found")]
    OptionNotFound {},

    #[error("Option already listed")]
    OptionAlreadyListed {},

    #[error("Invalid price")]
    InvalidPrice {},

    #[error("Option not listed")]
    OptionNotListed {},

    #[error("Insufficient amount")]
    InsufficientAmount {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Option not bought")]
    OptionNotBought {},

    // Add more specific error cases as needed
}