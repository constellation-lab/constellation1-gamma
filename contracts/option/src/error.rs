use cosmwasm_std::{Coin, StdError, Timestamp};
use thiserror::Error;

#[derive(Error,Debug)]
#[allow(dead_code)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("expired option (expired {expired:?})")]
    OptionExpired { expired: Timestamp },

    #[error("not expired option (expires {expires:?})")]
    OptionNotExpired { expires:  Timestamp},

    #[error("option not found")]
    OptionNotFound {},

    #[error("unauthorized")]
    Unauthorized {},

    #[error("No bid or Offer")]
    NoBidOrOffer,

    #[error("Invalid Option Status")]
    InvalidOptionStatus{},

    #[error("Invalid Fraction")]
    InvalidFraction{},

    #[error("Withdraw Before Expiration")]
    WithdrawBeforeExpiration {},

    #[error("Insuffient Funds")]
    InsufficientFunds {},

    #[error("Invalid Expiration")]
    InvalidExpiration {},

    #[error("Oracle Not Set")]
    OracleNotSet {},

    #[error("Config Not Found")]
    ConfigNotFound{},

    #[error("Invalid proposal vote")]
    InvalidVote {},
    
    #[error("Share overflow")]
    ShareOverflow {},  

    #[error("Invalid Expiration Notification")]
    InvalidExpirationNotification {},

    #[error("Json Serialization Error")]
    JsonSerializationError {},

    #[error("must send exact counter offer (offer {offer:?}, counter_offer: {counter_offer:?})")]
    CounterOfferMismatch {
        offer: Vec<Coin>,
        counter_offer: Vec<Coin>,
    },

    #[error("must send exact counter offer (offer {offer:?}, price: {price:?})")]
    PriceMismatch {
        offer: Vec<Coin>,
        price: Vec<Coin>,
    },

    #[error("must send exact bid amount expected (bid_amount {bid_amount:?},  expected_bid_amount: {expected_bid_amount:?})")]
    BidAmountMismatch {
        bid_amount:             Vec<Coin>,
        expected_bid_amount:    Vec<Coin>,
    },

    #[error("must send exact offer amount expected (offer_amount {offer_amount:?},  expected_offer_amount: {expected_offer_amount:?})")]
    OfferAmountMismatch {
        offer_amount:             Vec<Coin>,
        expected_offer_amount:    Vec<Coin>,
    },

    #[error("do not send funds with burn")]
    FundsSentWithBurn {},

    #[error("can't find the option")]
    OptionCanotFind{},

    #[error("can't find the option in the market")]
    OptionCanotFindInTheMarket{},

    #[error("Invalid criteria")]
    InvalidCriteria {},

    #[error("AMM pool not found")]
    PoolNotFound {},
    
    #[error("Invalid trade amount")]  
    InvalidTradeAmount {},
  
    #[error("Data feed error")]
    DataFeedError {},

    #[error("std error")]
    StdErr(StdError),

}

impl From<serde_json::Error> for ContractError {
    fn from(_err: serde_json::Error) -> Self {
      ContractError::JsonSerializationError {} 
    }
  }

impl PartialEq for ContractError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ContractError::NoBidOrOffer, ContractError::NoBidOrOffer) => true,
            _ => false,
        }
    }
}

impl Eq for ContractError {}


