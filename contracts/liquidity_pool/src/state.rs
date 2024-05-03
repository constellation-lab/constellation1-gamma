use cosmwasm_std::{Addr, Coin, Uint128};
use schemars::JsonSchema;
//use serde::{Deserialize, Serialize};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct LiquidityPool {
pub assets: Vec<Coin>,
pub lp_token_supply: Uint128,
}
#[cw_serde]
pub struct LiquidityProviderInfo {
pub address: Addr,
pub assets: Vec<Coin>,
pub lp_tokens: Uint128,
}
pub const LP_INFO: Map<Addr, LiquidityProviderInfo> = Map::new("lp_info");
pub const LP_TOKENS: Map<Addr, Uint128> = Map::new("lp_tokens");
pub const POOL: Item<LiquidityPool> = Item::new("pool");