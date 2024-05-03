use cosmwasm_std::{
    entry_point, to_json_binary, to_binary, Binary, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
#[allow(unused_imports)]
use crate::error::ContractError;
use crate::events::{PricingOracleEvent};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, CalculateOptionPriceResponse, PoolInfo};
use crate::state::{Config, CONFIG, OptionPrice, OPTION_PRICES};
use crate::query::{query_config, query_calculate_option_price, query_option_price};

const LIQUIDITY_POOL_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const CONSTELLA_OPTION_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin: deps.api.addr_validate(&msg.admin)?,
        liquidity_pool_contract: deps.api.addr_validate(&msg.liquidity_pool_contract)?,
        constella_option_contract: deps.api.addr_validate(&msg.constella_option_contract)?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { admin, liquidity_pool_contract, constella_option_contract } => {
            execute_update_config(deps, info, admin, liquidity_pool_contract, constella_option_contract)
        }
        ExecuteMsg::SaveOptionPrice { option_id, price } => {            
            execute_save_option_price(deps, option_id, price)        
        }
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    liquidity_pool_contract: Option<String>,
    constella_option_contract: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(admin) = admin {
        config.admin = deps.api.addr_validate(&admin)?;
    }

    if let Some(liquidity_pool_contract) = liquidity_pool_contract {
        config.liquidity_pool_contract = deps.api.addr_validate(&liquidity_pool_contract)?;
    }

    if let Some(constella_option_contract) = constella_option_contract {
        config.constella_option_contract = deps.api.addr_validate(&constella_option_contract)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

fn execute_save_option_price(
    deps: DepsMut,
    option_id: u64,
    price: Uint128,
) -> Result<Response, ContractError> {
    let option_price = OptionPrice {
        option_id,
        price,
    };
    OPTION_PRICES.save(deps.storage, option_id, &option_price)?;

    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::CalculateOptionPrice {
            option_id,
            collateral,
            counter_offer,
            expiration,
        } => {
            let option_price = query_calculate_option_price(deps, env, option_id, collateral, counter_offer, expiration)?;
            to_json_binary(&option_price)
        }
        QueryMsg::GetOptionPrice { option_id } => to_json_binary(&query_option_price(deps, option_id)?),
        QueryMsg::GetPoolInfo {} => to_json_binary(&query_pool_info(deps)?),
    }
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

pub fn calculate_option_price(
    deps: Deps,
    env: Env,
    collateral: Uint128,
    counter_offer: Uint128,
    expiration: u64,
    total_collateral: Uint128,
    total_counter_offer: Uint128,
    total_liquidity: Uint128,
) -> Result<Uint128, ContractError> {
    // Calculate the option price based on the AMM algorithm
    let constant_product = total_collateral * total_counter_offer;
    let new_total_collateral = total_collateral + collateral;
    let new_total_counter_offer = constant_product / new_total_collateral;
    let counter_offer_delta = total_counter_offer - new_total_counter_offer;

    // Calculate the premium based on the counter_offer_delta and expiration time
    let time_to_expiration = expiration - env.block.time.seconds();
    let premium_factor = cosmwasm_std::Decimal::from_ratio(counter_offer_delta, total_liquidity);
    let time_factor = cosmwasm_std::Decimal::from_ratio(time_to_expiration, 86400u64); // Assuming 1 day = 86400 seconds
    let premium = premium_factor * time_factor;

    // Calculate the final option price
    //let option_price = counter_offer + (premium * total_liquidity).into();
    //let option_price: Uint128 = counter_offer + (premium * total_liquidity).into();
    let option_price = counter_offer + Uint128::from((premium * total_liquidity).u128());
    


    Ok(option_price)
}


/*

use cosmwasm_std::Uint128;
use statistic::distribution::{Continuous, Normal};

fn calculate_option_price(
    collateral: Uint128,
    counter_offer: Uint128,
    expiration: u64,
    total_collateral: Uint128,
    total_counter_offer: Uint128,
    total_liquidity: Uint128,
    volatility: Decimal,
    risk_free_rate: Decimal,
) -> StdResult<Uint128> {
    // Calculate the option price based on the concentrated liquidity AMM algorithm
    // ...

    // Calculate the premium (option price) using the Black-Scholes model
    let time_to_expiration = Decimal::from_ratio(expiration - env.block.time.seconds(), 86400u64);
    let d1 = (Decimal::from_ratio(collateral, counter_offer) / volatility * time_to_expiration.sqrt()
        + (risk_free_rate + volatility * volatility / Decimal::from_ratio(2u128, 1u128))
        * time_to_expiration)
        / (volatility * time_to_expiration.sqrt());
    let d2 = d1 - volatility * time_to_expiration.sqrt();
    let normal = Normal::new(0.0, 1.0).unwrap();
    let call_price = collateral * normal.cdf(d1) - counter_offer * (-risk_free_rate * time_to_expiration).exp() * normal.cdf(d2);

    // Adjust the option price based on the calculated premium
    let option_price = amm_price + call_price;

    Ok(option_price)
}


*/

/*

use cosmwasm_std::{Decimal, Uint128};
use std::collections::BTreeMap;

struct LiquidityRange {
    lower_bound: Decimal,
    upper_bound: Decimal,
    liquidity: Uint128,
    collateral_per_liquidity: Decimal,
    counter_offer_per_liquidity: Decimal,
}

fn calculate_option_price_amm(
    collateral: Uint128,
    counter_offer: Uint128,
    expiration: u64,
    liquidity_ranges: &mut BTreeMap<Decimal, LiquidityRange>,
    fee_rate: Decimal,
) -> StdResult<Uint128> {
    // Find the active liquidity range based on the current price
    let current_price = Decimal::from_ratio(collateral, counter_offer);
    let (active_range_lower_bound, active_range) = liquidity_ranges
        .range(..=current_price)
        .next_back()
        .ok_or_else(|| StdError::generic_err("No active liquidity range found"))?;

    // Calculate the option price within the active liquidity range
    let liquidity = active_range.liquidity;
    let collateral_per_liquidity = active_range.collateral_per_liquidity;
    let counter_offer_per_liquidity = active_range.counter_offer_per_liquidity;

    let collateral_amount = collateral;
    let counter_offer_amount = counter_offer;

    let new_collateral_per_liquidity = collateral_per_liquidity + Decimal::from_ratio(collateral_amount, liquidity);
    let new_counter_offer_per_liquidity = counter_offer_per_liquidity - Decimal::from_ratio(counter_offer_amount, liquidity);

    let price_lower = *active_range_lower_bound;
    let price_upper = active_range.upper_bound;

    let new_price = if current_price < price_lower {
        price_lower
    } else if current_price > price_upper {
        price_upper
    } else {
        current_price
    };

    // Update the liquidity range with the new price and liquidity amounts
    let updated_range = LiquidityRange {
        lower_bound: active_range.lower_bound,
        upper_bound: active_range.upper_bound,
        liquidity,
        collateral_per_liquidity: new_collateral_per_liquidity,
        counter_offer_per_liquidity: new_counter_offer_per_liquidity,
    };
    liquidity_ranges.insert(active_range_lower_bound.clone(), updated_range);

    // Calculate the fees based on the trade volume and fee rate
    let trade_volume = collateral_amount;
    let fees = trade_volume * fee_rate;

    // Calculate the option price considering the fees
    let option_price = counter_offer_amount + fees;

    Ok(option_price)
}

*/

/* with more checks of edge cases


use cosmwasm_std::{Decimal, Uint128, StdError, StdResult};
use std::collections::BTreeMap;

struct LiquidityRange {
    // ... (same as before)
}

fn calculate_option_price_amm(
    collateral: Uint128,
    counter_offer: Uint128,
    expiration: u64,
    liquidity_ranges: &mut BTreeMap<Decimal, LiquidityRange>,
    fee_rate: Decimal,
    min_liquidity: Uint128,
    max_price_impact: Decimal,
) -> StdResult<Uint128> {
    // Check if collateral and counter offer amounts are greater than zero
    if collateral.is_zero() || counter_offer.is_zero() {
        return Err(StdError::generic_err("Collateral and counter offer amounts must be greater than zero"));
    }

    // Check if expiration is in the future
    let current_time = env.block.time.seconds();
    if expiration <= current_time {
        return Err(StdError::generic_err("Expiration must be in the future"));
    }

    // Find the active liquidity range based on the current price
    let current_price = Decimal::from_ratio(collateral, counter_offer);
    let (active_range_lower_bound, active_range) = liquidity_ranges
        .range(..=current_price)
        .next_back()
        .ok_or_else(|| StdError::generic_err("No active liquidity range found"))?;

    // Check if there is sufficient liquidity in the active range
    if active_range.liquidity < min_liquidity {
        return Err(StdError::generic_err("Insufficient liquidity in the active range"));
    }

    // Calculate the option price within the active liquidity range
    let liquidity = active_range.liquidity;
    let collateral_per_liquidity = active_range.collateral_per_liquidity;
    let counter_offer_per_liquidity = active_range.counter_offer_per_liquidity;

    let collateral_amount = collateral;
    let counter_offer_amount = counter_offer;

    let new_collateral_per_liquidity = collateral_per_liquidity + Decimal::from_ratio(collateral_amount, liquidity);
    let new_counter_offer_per_liquidity = counter_offer_per_liquidity - Decimal::from_ratio(counter_offer_amount, liquidity);

    let price_lower = *active_range_lower_bound;
    let price_upper = active_range.upper_bound;

    let new_price = if current_price < price_lower {
        price_lower
    } else if current_price > price_upper {
        price_upper
    } else {
        current_price
    };

    // Check if the price impact is within the acceptable range
    let price_impact = (new_price - current_price).abs() / current_price;
    if price_impact > max_price_impact {
        return Err(StdError::generic_err("Price impact exceeds the acceptable range"));
    }

    // Update the liquidity range with the new price and liquidity amounts
    let updated_range = LiquidityRange {
        lower_bound: active_range.lower_bound,
        upper_bound: active_range.upper_bound,
        liquidity,
        collateral_per_liquidity: new_collateral_per_liquidity,
        counter_offer_per_liquidity: new_counter_offer_per_liquidity,
    };
    liquidity_ranges.insert(active_range_lower_bound.clone(), updated_range);

    // Calculate the fees based on the trade volume and fee rate
    let trade_volume = collateral_amount;
    let fees = trade_volume * fee_rate;

    // Calculate the option price considering the fees
    let option_price = counter_offer_amount + fees;

    Ok(option_price)
}



*/