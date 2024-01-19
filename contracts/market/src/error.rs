use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("UnauthorizedOwner")]
    UnauthorizedOwner {},

    #[error("OptionIsburnedOrexpired")]
    OptionIsburnedOrexpired {},

    #[error("UnauthorizedOperator")]
    UnauthorizedOperator {},

    #[error("InvalidPrice")]
    InvalidPrice {},

    #[error("ListItemExpired")]
    ListItemIsExpired {},

    #[error("ListItemNotActive")]
    AskNotActive {},

    #[error("ListItemNotFound")]
    AskNotFound {},

    #[error("InvalidToken")]
    InvalidToken {},


    #[error("BidExpired")]
    BidExpired {},

}
