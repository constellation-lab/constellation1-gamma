use cosmwasm_std::{Addr, Uint128};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Event, StdResult};

#[cw_serde]
pub enum IncentiveManagerEvent {
    YieldFarmingProgramCreated {
        program_id: String,
        reward_token: String,
        reward_rate: Uint128,
        start_time: u64,
        end_time: u64,
    },
    LiquidityMiningProgramCreated {
        program_id: String,
        option_pair: String,
        reward_multiplier: Uint128,
        start_time: u64,
        end_time: u64,
    },
    TokensStaked {
        program_id: String,
        user: Addr,
        amount: Uint128,
    },
    TokensUnstaked {
        program_id: String,
        user: Addr,
        amount: Uint128,
    },
    RewardsClaimed {
        program_id: String,
        user: Addr,
        amount: Uint128,
    },
    PerformanceFeesDistributed {
        amount: Uint128,
    },
}

impl IncentiveManagerEvent {
    pub fn into_event(self) -> StdResult<Event> {
        match self {
            IncentiveManagerEvent::YieldFarmingProgramCreated {
                program_id,
                reward_token,
                reward_rate,
                start_time,
                end_time,
            } => Ok(Event::new("yield_farming_program_created")
                .add_attribute("program_id", program_id)
                .add_attribute("reward_token", reward_token)
                .add_attribute("reward_rate", reward_rate.to_string())
                .add_attribute("start_time", start_time.to_string())
                .add_attribute("end_time", end_time.to_string())),
            IncentiveManagerEvent::LiquidityMiningProgramCreated {
                program_id,
                option_pair,
                reward_multiplier,
                start_time,
                end_time,
            } => Ok(Event::new("liquidity_mining_program_created")
                .add_attribute("program_id", program_id)
                .add_attribute("option_pair", option_pair)
                .add_attribute("reward_multiplier", reward_multiplier.to_string())
                .add_attribute("start_time", start_time.to_string())
                .add_attribute("end_time", end_time.to_string())),
            IncentiveManagerEvent::TokensStaked {
                program_id,
                user,
                amount,
            } => Ok(Event::new("tokens_staked")
                .add_attribute("program_id", program_id)
                .add_attribute("user", user.to_string())
                .add_attribute("amount", amount.to_string())),
            IncentiveManagerEvent::TokensUnstaked {
                program_id,
                user,
                amount,
            } => Ok(Event::new("tokens_unstaked")
                .add_attribute("program_id", program_id)
                .add_attribute("user", user.to_string())
                .add_attribute("amount", amount.to_string())),
            IncentiveManagerEvent::RewardsClaimed {
                program_id,
                user,
                amount,
            } => Ok(Event::new("rewards_claimed")
                .add_attribute("program_id", program_id)
                .add_attribute("user", user.to_string())
                .add_attribute("amount", amount.to_string())),
            IncentiveManagerEvent::PerformanceFeesDistributed { amount } => {
                Ok(Event::new("performance_fees_distributed")
                    .add_attribute("amount", amount.to_string()))
            }
        }
    }
}