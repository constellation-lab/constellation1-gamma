use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

pub const CONFIG: Item<TokenInfo> = Item::new("token_info");
pub const STAKE_INFO: Map<Addr, StakeInfo> = Map::new("stake_info");

#[cw_serde]
pub struct StakeInfo {
    pub staked_amount: Uint128,
}