use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub liquidity_pool_contract: String,
    pub pricing_oracle_contract: String,
    pub constella_option_contract: String,
    pub option_marketplace_contract: String,
    pub incentive_manager_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPositionLimit {
        option_pair: String,
        max_position: Uint128,
    },
    SetCircuitBreaker {
        option_pair: String,
        price_threshold: Decimal,
        triggered: bool,
    },
    ExecuteRiskMitigationStrategy {},
    AdjustPricing {
        option_pair: String,
        adjustment_factor: Decimal,
    },
    ClosePosition {
        option_pair: String,
        amount: Uint128,
    },
    AdjustParameters {
        volatility_multiplier: Decimal,
    },
    
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    PositionLimit {
        option_pair: String,
    },
    CircuitBreaker {
        option_pair: String,
    },
    GetPoolInfo {},
    GetOptionPositions {},
    GetPrice {
        option_pair: String,
    },
}

#[cw_serde]
pub struct PoolInfo {
    pub total_collateral: Uint128,
    pub total_liquidity: Uint128,
    // Add more fields as needed
}

#[cw_serde]
pub struct OptionPosition {
    pub option_pair: String,
    pub amount: Uint128,
    // Add more fields as needed
}