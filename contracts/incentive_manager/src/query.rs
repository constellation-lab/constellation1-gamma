use cosmwasm_std::{StdResult, Deps};
use crate::msg::{QueryMsg, PoolInfo};
use crate::state::{CONFIG, YIELD_FARMING_PROGRAMS, LIQUIDITY_MINING_PROGRAMS, USER_STAKED_AMOUNT, USER_REWARD_PER_TOKEN_PAID, TOTAL_FEES_DISTRIBUTED};

pub fn query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::YieldFarmingProgram { program_id } => to_binary(&query_yield_farming_program(deps, program_id)?),
        QueryMsg::LiquidityMiningProgram { program_id } => to_binary(&query_liquidity_mining_program(deps, program_id)?),
        QueryMsg::UserStakedAmount { user, program_id } => to_binary(&query_user_staked_amount(deps, user, program_id)?),
        QueryMsg::UserRewardPerTokenPaid { user, program_id } => to_binary(&query_user_reward_per_token_paid(deps, user, program_id)?),
        QueryMsg::TotalFeesDistributed {} => to_binary(&query_total_fees_distributed(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_yield_farming_program(deps: Deps, program_id: String) -> StdResult<YieldFarmingProgram> {
    let program = YIELD_FARMING_PROGRAMS.load(deps.storage, program_id)?;
    Ok(program)
}

fn query_liquidity_mining_program(deps: Deps, program_id: String) -> StdResult<LiquidityMiningProgram> {
    let program = LIQUIDITY_MINING_PROGRAMS.load(deps.storage, program_id)?;
    Ok(program)
}

fn query_user_staked_amount(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (&user_addr, &program_id)).unwrap_or_default();
    Ok(staked_amount)
}

fn query_user_reward_per_token_paid(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let reward_per_token_paid = USER_REWARD_PER_TOKEN_PAID.load(deps.storage, (&user_addr, &program_id)).unwrap_or_default();
    Ok(reward_per_token_paid)
}

fn query_total_fees_distributed(deps: Deps) -> StdResult<Uint128> {
    let total_fees_distributed = TOTAL_FEES_DISTRIBUTED.load(deps.storage)?;
    Ok(total_fees_distributed)
}