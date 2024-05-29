use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub constella_option_contract: Addr,
    pub incentive_manager_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct StakeInfo {
    pub staked_amount: Uint128,
}

pub const STAKE_INFO: Map<Addr, StakeInfo> = Map::new("stake_info");


/*use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};


pub const CONFIG: Item<Config> = Item::new("config");

pub struct Config {
    pub constella_option_contract: Addr,
    pub incentive_manager_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MinterData {
    pub minter: Addr,
    pub cap: Option<Uint128>,
}

pub const CONFIG: Item<TokenInfo> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct StakeInfo {
    pub staked_amount: Uint128,
}

pub const STAKE_INFO: Map<Addr, StakeInfo> = Map::new("stake_info");*/