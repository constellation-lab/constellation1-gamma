use cosmwasm_std::{ StdError, Timestamp, Coin};
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

    #[error("counter offer mismatch (offer{offer:?}, counter_offer{counter_offer:?})")]
    CounterOfferMismatch {offer: Coin,counter_offer:Coin},


    #[error("option not found")]
    OptionNotFound {},

    #[error("option is burned")]
    OptionIsBurned {},

    #[error("unauthorized")]
    Unauthorized {},

    #[error("split can't over 100")]
    Over100 {},


    #[error("No bid or Offer")]
    NoBidOrOffer,

    #[error("Invalid Option Status")]
    InvalidOptionStatus{},

    #[error("Invalid Fraction")]
    InvalidFraction{},

    #[error("Invalid Expiration")]
    InvalidExpiration {},


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

