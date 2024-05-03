use cosmwasm_std::{Binary, to_json_binary, Deps, Env, StdResult, StdError, to_binary, Uint128};
#[allow(unused_imports)]
use crate::msg::{QueryMsg, ConfigResponse, CalculateOptionPriceResponse, PoolInfo};
use crate::state::{CONFIG, OPTION_PRICES, OptionPrice};
use crate::pricing_oracle::calculate_option_price;
use cosmwasm_std::DepsMut;


pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::CalculateOptionPrice {
            option_id,
            collateral,
            counter_offer,
            expiration,
        } => to_json_binary(&query_calculate_option_price(
            deps,
            env,
            option_id,
            collateral,
            counter_offer,
            expiration,
        )?),
        QueryMsg::GetOptionPrice { option_id } => to_json_binary(&query_option_price(deps, option_id)?),
        QueryMsg::GetPoolInfo {} => to_json_binary(&query_pool_info(deps)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admin: config.admin,
        liquidity_pool_contract: config.liquidity_pool_contract,
        constella_option_contract: config.constella_option_contract,
    })
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


pub fn query_calculate_option_price(
    deps: Deps,
    env: Env,
    option_id: u64,
    collateral: Uint128,
    counter_offer: Uint128,
    expiration: u64,
) -> StdResult<OptionPrice> {
    let config = CONFIG.load(deps.storage)?;
    let liquidity_pool_contract = config.liquidity_pool_contract;

    // Query the liquidity pool for the current pool composition
    let pool_info: PoolInfo = deps.querier.query_wasm_smart(
        liquidity_pool_contract,
        &QueryMsg::GetPoolInfo {},
    )?;

    // Calculate the option price based on the pool composition and user parameters
    let price = calculate_option_price(
        deps,
        env,
        collateral,
        counter_offer,
        expiration,
        pool_info.total_collateral,
        pool_info.total_counter_offer,
        pool_info.total_liquidity,
    ).map_err(|err| StdError::generic_err(err.to_string()))?;

    // Create the OptionPrice struct
    let option_price = OptionPrice {
        option_id,
        price,
    };

    Ok(option_price)
}

pub fn query_option_price(deps: Deps, option_id: u64) -> StdResult<OptionPrice> {
    let option_price = OPTION_PRICES.load(deps.storage, option_id)?;
    Ok(option_price)
}


