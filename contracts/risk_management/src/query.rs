use cosmwasm_std::{StdResult, Deps, Binary, to_binary, StdError, Decimal};
use crate::msg::{QueryMsg, PoolInfo, OptionPosition};
use crate::state::{Config, PositionLimit, CircuitBreaker, CONFIG, POSITION_LIMITS, CIRCUIT_BREAKERS};

pub fn query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::PositionLimit { option_pair } => to_binary(&query_position_limit(deps, option_pair)?),
        QueryMsg::CircuitBreaker { option_pair } => to_binary(&query_circuit_breaker(deps, option_pair)?),
        _ => Err(StdError::not_found("Query not supported")),
        QueryMsg::GetPoolInfo {} => {
            // Handle the GetPoolInfo query
            let pool_info = query_pool_info(deps)?;
            to_binary(&pool_info)
        }
        QueryMsg::GetOptionPositions {} => {
            // Handle the GetOptionPositions query
            let option_positions = query_option_positions(deps)?;
            to_binary(&option_positions)
        }
        QueryMsg::GetPrice { option_pair } => {
            // Handle the GetPrice query
            let price = query_price(deps, option_pair)?;
            to_binary(&price)
        }
    }
}


fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_position_limit(deps: Deps, option_pair: String) -> StdResult<PositionLimit> {
    let position_limit = POSITION_LIMITS.load(deps.storage, option_pair)?;
    Ok(position_limit)
}

fn query_circuit_breaker(deps: Deps, option_pair: String) -> StdResult<CircuitBreaker> {
    let circuit_breaker = CIRCUIT_BREAKERS.load(deps.storage, option_pair)?;
    Ok(circuit_breaker)
}


fn query_pool_info(deps: Deps) -> StdResult<PoolInfo> {
    let config = CONFIG.load(deps.storage)?;
    let pool_info: PoolInfo = deps.querier.query_wasm_smart(
        &config.liquidity_pool_contract,
        &QueryMsg::GetPoolInfo {},
    )?;
    Ok(pool_info)
}

fn query_option_positions(deps: Deps) -> StdResult<Vec<OptionPosition>> {
    let config = CONFIG.load(deps.storage)?;
    let option_positions: Vec<OptionPosition> = deps.querier.query_wasm_smart(
        &config.liquidity_pool_contract,
        &QueryMsg::GetOptionPositions {},
    )?;
    Ok(option_positions)
}

fn query_price(deps: Deps, option_pair: String) -> StdResult<Decimal> {
    let config = CONFIG.load(deps.storage)?;
    let price: Decimal = deps.querier.query_wasm_smart(
        &config.pricing_oracle_contract,
        &QueryMsg::GetPrice { option_pair },
    )?;
    Ok(price)
}