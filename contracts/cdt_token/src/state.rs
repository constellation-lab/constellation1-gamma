use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
//use cosmwasm_schema::serde::{Deserialize, Serialize};
//use cosmwasm_schema::serde;
use serde::{Deserialize, Serialize};


#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

pub const CONFIG: Item<TokenInfo> = Item::new("token_info");
pub const STAKE_INFO: Map<Addr, StakeInfo> = Map::new("stake_info");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub yes_votes: Uint128,
    pub no_votes: Uint128,
    pub voting_end_time: u64,
    pub total_yes_votes: Uint128,
    pub total_no_votes: Uint128,
    pub votes: Vec<(Addr, bool)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StakeInfo {
    pub staked_amount: Uint128,
}