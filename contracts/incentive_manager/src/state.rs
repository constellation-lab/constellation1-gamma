use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub governance_token: Addr,
    pub liquidity_pool_contract: Addr,
    pub constella_option_contract: Addr,
    pub pricing_oracle_contract: Addr,
    pub option_marketplace_contract: Addr,
    pub fee_distributor: Addr,
}

#[cw_serde]
pub struct YieldFarmingProgram {
    pub program_id: String,
    pub reward_token: String,
    pub reward_rate: Uint128,
    pub start_time: u64,
    pub end_time: u64,
    pub total_staked: Uint128,
    pub reward_per_token_stored: Uint128,
    pub last_update_time: u64,
}

#[cw_serde]
pub struct LiquidityMiningProgram {
    pub program_id: String,
    pub option_pair: String,
    pub reward_multiplier: Uint128,
    pub start_time: u64,
    pub end_time: u64,
    pub total_liquidity: Uint128,
    pub reward_per_token_stored: Uint128,
    pub last_update_time: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const YIELD_FARMING_PROGRAMS: Map<String, YieldFarmingProgram> = Map::new("yield_farming_programs");
pub const LIQUIDITY_MINING_PROGRAMS: Map<String, LiquidityMiningProgram> = Map::new("liquidity_mining_programs");
pub const USER_STAKED_AMOUNT: Map<(&Addr, &String), Uint128> = Map::new("user_staked_amount");
pub const USER_REWARD_PER_TOKEN_PAID: Map<(&Addr, &String), Uint128> = Map::new("user_reward_per_token_paid");
pub const LIQUIDITY_PROVIDERS: Map<Addr, Uint128> = Map::new("liquidity_providers");
pub const TOTAL_FEES_DISTRIBUTED: Item<Uint128> = Item::new("total_fees_distributed");