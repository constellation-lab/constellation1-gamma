use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin,Decimal, Uint128};
use crate::msg::OptionResponse;
use cw_storage_plus::Map;

#[cw_serde]
pub struct Option {
    pub option_id: u64,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub expiration: u64,
    pub is_listed: bool,
    pub price: Uint128,
    pub price_denom: String,
    pub min_trade_amount: Uint128,
    pub buyer: Addr,
    pub amount: Uint128,
    pub slippage_tolerance: Decimal,
}

impl From<Option> for OptionResponse {
    fn from(option: Option) -> Self {
        OptionResponse {
            option_id: option.option_id,
            owner: option.owner,
            collateral: option.collateral,
            counter_offer: option.counter_offer,
            expiration: option.expiration,
            is_listed: option.is_listed,
            price: option.price,
            price_denom: option.price_denom,
            min_trade_amount: option.min_trade_amount,
            buyer: option.buyer,
            amount: option.amount,
        }
    }
}

pub const OPTIONS: Map<u64, Option> = Map::new("options");