use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { recipient: String, amount: Uint128 },
    Burn { amount: Uint128 },
    Transfer { recipient: String, amount: Uint128 },
    Stake { amount: Uint128 },
}

#[cw_serde]
pub enum QueryMsg {
    Balance { address: String },
    TokenInfo {},
    StakeInfo { address: String },
}

#[cw_serde]
pub struct Cw20Coin {
    pub address: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

#[cw_serde]
pub struct StakeInfo {
    pub staked_amount: Uint128,
}