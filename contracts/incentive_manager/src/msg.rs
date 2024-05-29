use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128, Addr};
use cw20::{Cw20ReceiveMsg, Cw20Coin, MinterResponse};
use cw20_base::msg::InstantiateMarketingInfo;

#[cw_serde]
pub struct InstantiateMsg {
    // Standard CW20 fields
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,

     // Custom fields for the incentive manager contract
     pub governance_token: Addr, //remove
     pub liquidity_pool_contract: String,
     pub constella_option_contract: String,
     pub pricing_oracle_contract: String,
     pub option_marketplace_contract: String,
     pub cdt_token_contract: String, //this is the governance token
     pub cpst_token_contract: String,
     pub cnste_token_contract: String,
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
    
    CreateCnsteStakingProgram {
        program_id: String,
        reward_per_token_stored: Uint128,
        last_update_time: u64,
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
    
    GetPoolInfo {},


    CnsteStakingProgram { 
        program_id: String 
    },
    UserCnsteStakedAmount { 
        user: String, 
        program_id: String 
    },
    TotalCnsteStakedAmount { 
        program_id: String 
    },
    CnsteRewardPool {},

}

#[cw_serde]
pub struct PoolInfo {
    pub total_liquidity: Uint128,
}


//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct CnsteStakingProgramInfo {
    pub program_id: String,
    pub reward_per_token_stored: Uint128,
    pub last_update_time: u64,
}

