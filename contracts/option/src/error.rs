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

    #[error("unauthorized")]
    Unauthorized {},

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

    #[error("do not send funds with burn")]
    FundsSentWithBurn {},

    #[error("can't find the option")]
    OptionCanotFind{},

    #[error("can't find the option in the market")]
    OptionCanotFindInTheMarket{},

}
