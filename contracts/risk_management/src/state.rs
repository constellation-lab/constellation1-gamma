use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub liquidity_pool_contract: Addr,
    pub pricing_oracle_contract: Addr,
    pub constella_option_contract: Addr,
    pub option_marketplace_contract: Addr,
    pub incentive_manager_contract: Addr,
}

#[cw_serde]
pub struct PositionLimit {
    pub option_pair: String,
    pub max_position: Uint128,
}

#[cw_serde]
pub struct CircuitBreaker {
    pub option_pair: String,
    pub price_threshold: Decimal,
    pub triggered: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POSITION_LIMITS: Map<String, PositionLimit> = Map::new("position_limits");
pub const CIRCUIT_BREAKERS: Map<String, CircuitBreaker> = Map::new("circuit_breakers");