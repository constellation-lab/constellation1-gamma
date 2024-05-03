use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub liquidity_pool_contract: String,
    pub constella_option_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        liquidity_pool_contract: Option<String>,
        constella_option_contract: Option<String>,
    },

    SaveOptionPrice {
        option_id: u64,
        price: Uint128,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    CalculateOptionPrice {
        option_id: u64,
        collateral: Uint128,
        counter_offer: Uint128,
        expiration: u64,
    },
    GetOptionPrice {
        option_id: u64,
    },
    GetPoolInfo {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: Addr,
    pub liquidity_pool_contract: Addr,
    pub constella_option_contract: Addr,
}

#[cw_serde]
pub struct CalculateOptionPriceResponse {
    pub price: Uint128,
}

#[cw_serde]
pub struct PoolInfo {
    pub total_collateral: Uint128,
    pub total_counter_offer: Uint128,
    pub total_liquidity: Uint128,
}

/*use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub liquidity_pool_contract: String,
    pub constella_option_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        liquidity_pool_contract: Option<String>,
        constella_option_contract: Option<String>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    CalculateOptionPrice {
        collateral: Uint128,
        counter_offer: Uint128,
        expiration: u64,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: Addr,
    pub liquidity_pool_contract: Addr,
    pub constella_option_contract: Addr,
}

#[cw_serde]
pub struct CalculateOptionPriceResponse {
    pub price: Uint128,
}*/