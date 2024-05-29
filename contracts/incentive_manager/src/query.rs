use cosmwasm_std::{StdResult, Deps, Binary, to_binary, Uint128};
use crate::msg::{QueryMsg, PoolInfo, CnsteStakingProgramInfo};
use crate::state::Config;
use crate::state::YieldFarmingProgram;
use crate::state::LiquidityMiningProgram;
use crate::state::{CONFIG, YIELD_FARMING_PROGRAMS, LIQUIDITY_MINING_PROGRAMS, USER_STAKED_AMOUNT, USER_REWARD_PER_TOKEN_PAID, TOTAL_FEES_DISTRIBUTED};
use crate::state::{CNSTE_STAKING_PROGRAMS, USER_CNSTE_STAKED_AMOUNT, TOTAL_CNSTE_STAKED_AMOUNT, CNSTE_REWARD_POOL};
    

pub fn query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::YieldFarmingProgram { program_id } => to_binary(&query_yield_farming_program(deps, program_id)?),
        QueryMsg::LiquidityMiningProgram { program_id } => to_binary(&query_liquidity_mining_program(deps, program_id)?),
        QueryMsg::UserStakedAmount { user, program_id } => to_binary(&query_user_staked_amount(deps, user, program_id)?),
        QueryMsg::UserRewardPerTokenPaid { user, program_id } => to_binary(&query_user_reward_per_token_paid(deps, user, program_id)?),
        QueryMsg::TotalFeesDistributed {} => to_binary(&query_total_fees_distributed(deps)?),
        QueryMsg::GetPoolInfo {} => to_binary(&query_pool_info(deps)?),

        QueryMsg::CnsteStakingProgram { program_id } => to_binary(&query_cnste_staking_program(deps, program_id)?),
        QueryMsg::UserCnsteStakedAmount { user, program_id } => to_binary(&query_user_cnste_staked_amount(deps, user, program_id)?),
        QueryMsg::TotalCnsteStakedAmount { program_id } => to_binary(&query_total_cnste_staked_amount(deps, program_id)?),
        QueryMsg::CnsteRewardPool {} => to_binary(&query_cnste_reward_pool(deps)?),

        
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
    let staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (user_addr, program_id)).unwrap_or_default();
    Ok(staked_amount)
}

fn query_user_reward_per_token_paid(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let reward_per_token_paid = USER_REWARD_PER_TOKEN_PAID.load(deps.storage, (user_addr, program_id)).unwrap_or_default();
    Ok(reward_per_token_paid)
}

fn query_total_fees_distributed(deps: Deps) -> StdResult<Uint128> {
    TOTAL_FEES_DISTRIBUTED.load(deps.storage)
}

fn query_pool_info(deps: Deps) -> StdResult<PoolInfo> {
    let config = CONFIG.load(deps.storage)?;
    let liquidity_pool_contract = config.liquidity_pool_contract;
    // Query the liquidity pool for the current pool composition
    let pool_info: PoolInfo = deps.querier.query_wasm_smart(
        liquidity_pool_contract,
        &QueryMsg::GetPoolInfo {},
    )?;
    Ok(pool_info)
}



fn query_cnste_staking_program(deps: Deps, program_id: String) -> StdResult<CnsteStakingProgramInfo> {
    let program = CNSTE_STAKING_PROGRAMS.load(deps.storage, program_id)?;
    Ok(CnsteStakingProgramInfo {
        program_id: program.program_id,
        reward_per_token_stored: program.reward_per_token_stored,
        last_update_time: program.last_update_time,
    })
}

fn query_user_cnste_staked_amount(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let staked_amount = USER_CNSTE_STAKED_AMOUNT.load(deps.storage, (user_addr, program_id)).unwrap_or_default();
    Ok(staked_amount)
}

fn query_total_cnste_staked_amount(deps: Deps, program_id: String) -> StdResult<Uint128> {
    let total_staked_amount = TOTAL_CNSTE_STAKED_AMOUNT.load(deps.storage, program_id).unwrap_or_default();
    Ok(total_staked_amount)
}

fn query_cnste_reward_pool(deps: Deps) -> StdResult<Uint128> {
    let cnste_reward_pool = CNSTE_REWARD_POOL.load(deps.storage)?;
    Ok(cnste_reward_pool)
}
