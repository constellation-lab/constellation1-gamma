use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use crate::state::Option;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ListOption {
        option_id: u64,
        price: Uint128,
        slippage_tolerance: Decimal,
        min_trade_amount: Uint128,
    },
    BuyOption {
        option_id: u64,
        amount: Uint128,
    },
    ExecuteOption {
        option_id: u64,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetOption {
        option_id: u64,
    },
    GetOptionPrice {
        option_id: u64,
        slippage_tolerance: Decimal,
    },
    ListOptions {},
}

#[cw_serde]
pub struct OptionResponse {
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
}

#[cw_serde]
pub struct OptionsResponse {
    pub options: Vec<OptionResponse>,
}

#[cw_serde]
pub enum PricingOracleQueryMsg {
    GetOptionPrice {
        option_id: u64,
        slippage_tolerance: Decimal,
    },
}
