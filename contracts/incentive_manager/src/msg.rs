use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128, Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub governance_token: Addr,
    pub liquidity_pool_contract: String,
    pub constella_option_contract: String,
    pub pricing_oracle_contract: String,
    pub option_marketplace_contract: String,
    pub fee_distributor: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateYieldFarmingProgram {
        program_id: String,
        reward_token: String,
        reward_rate: Uint128,
        start_time: u64,
        end_time: u64,
    },
    CreateLiquidityMiningProgram {
        program_id: String,
        option_pair: String,
        reward_multiplier: Uint128,
        start_time: u64,
        end_time: u64,
    },
    Stake {
        program_id: String,
        amount: Uint128,
    },
    Unstake {
        program_id: String,
        amount: Uint128,
    },
    ClaimRewards {
        program_id: String,
    },
    DistributePerformanceFees {
        amount: Uint128,
    },
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    YieldFarmingProgram {
        program_id: String,
    },
    LiquidityMiningProgram {
        program_id: String,
    },
    UserStakedAmount {
        user: String,
        program_id: String,
    },
    UserRewardPerTokenPaid {
        user: String,
        program_id: String,
    },
    TotalFeesDistributed {},
}

#[cw_serde]
pub struct PoolInfo {
    pub total_liquidity: Uint128,
}