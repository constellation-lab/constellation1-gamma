
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub liquidity_pool_contract: Addr,
    pub constella_option_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct OptionPrice {
    pub option_id: u64,
    pub price: Uint128,
}

pub const OPTION_PRICES: Map<u64, OptionPrice> = Map::new("option_prices");


/*use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub liquidity_pool_contract: Addr,
    pub constella_option_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");*/