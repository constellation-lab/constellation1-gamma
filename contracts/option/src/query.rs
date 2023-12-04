use cosmwasm_std::{Binary, Deps, Env, StdResult};
use std::collections::HashMap;
use crate::state::{CONFIG,OPTION_LIST,CREATOR_LIST,OWNER_LIST,MARKET_LIST, DISCOUNTS};
use crate::state::{Bid, BidOrOfferResponse, AcceptanceStatusResponse, DiscountCriteria};
use crate::state::{
    OptionStatusResponse, PartialExecutionResponse, MarketOptionPriceResponse,
    CollateralUsageResponse, CollateralUsageInfo, YieldFarmingInfoResponse, YieldFarmingInfo,
    DataFeedIntegrationResponse, DataFeedIntegrationInfo, DiscountCriteriaResponse,
};
use cosmwasm_std::{Uint128, Addr, QueryRequest, WasmQuery, Decimal};
use crate::msg::{ConfigResponse, QueryMsg}; 
use crate::msg::{OptionsResponse, GetOptionByIdResponse};
use cosmwasm_std::{
    entry_point, to_json_binary, Coin,Order};
use cw_storage_plus::Bound;
use maplit::hashmap;


//Queries
#[entry_point]
#[allow(dead_code)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::Options {}=> to_json_binary(&query_options(deps)?),
        QueryMsg::OptionsPage { key, amount }=>to_json_binary(&query_options_page(deps, key, amount)?),
        QueryMsg::GetOptionByid { id }=>to_json_binary(&query_option_by_id(deps,id)?),
        QueryMsg::MarketOptions {} =>to_json_binary(&query_market_options(deps)?),
        QueryMsg::MaketOptionsPagee { key, amount }=>to_json_binary(&query_market_options_page(deps, key, amount)?),
        QueryMsg::OwnerOptions { addr }=>to_json_binary(&query_owner_options(deps, addr)?),
        QueryMsg::CreateorOptions { addr }=>to_json_binary(&query_creator_options(deps, addr)?),
        
        QueryMsg::BidOrOffer { id }=>to_json_binary(&query_bid_or_offer(deps, id)?),
        QueryMsg::AcceptanceStatus { id }=>to_json_binary(&query_acceptance_status(deps, id)?),
        QueryMsg::BidHistory { option_id }=>to_json_binary(&query_bid_history(deps, option_id)?), 
        QueryMsg::BidAmount { id }=>to_json_binary(&query_bid_amount(deps, id)?),
        QueryMsg::OfferAmount { id }=>to_json_binary(&query_offer_amount(deps, id)?),
        QueryMsg::LatestPrice {oracle} => to_json_binary(&query_latest_price(deps, oracle)?),

        QueryMsg::OptionStatus { id } => to_json_binary(&query_option_status(deps, id)?),
        QueryMsg::PartialExecution { id } => to_json_binary(&query_partial_execution(deps, id)?),
        QueryMsg::MarketOptionPrice { id } => to_json_binary(&query_market_option_price(deps, id)?),
        QueryMsg::OptionExerciseConditions { id } => to_json_binary(&query_option_exercise_conditions(deps, id)?),
        QueryMsg::OptionParameters { id } => to_json_binary(&query_option_parameters(deps, id)?),
        QueryMsg::OptionExpiryNotification { id } => to_json_binary(&query_option_expiry_notification(deps, id)?),
        QueryMsg::OptionHistory { id } => to_json_binary(&query_option_history(deps, id)?),
        QueryMsg::OptionRiskMetrics { id } => to_json_binary(&query_option_risk_metrics(deps, id)?),
        QueryMsg::LiquidityProvided { id } => to_json_binary(&query_liquidity_provided(deps, id)?),
        QueryMsg::LiquidityWithdrawn { id } => to_json_binary(&query_liquidity_withdrawn(deps, id)?),
        QueryMsg::OptionGovernanceProposal { id } => to_json_binary(&query_option_governance_proposal(deps, id)?),
        QueryMsg::OptionCollateralUsage { id } => to_json_binary(&query_option_collateral_usage(deps, id)?),
        QueryMsg::AMMPoolDetails { id } => to_json_binary(&query_amm_pool_details(deps, id)?),
        QueryMsg::CollateralUsage { id } => to_json_binary(&query_collateral_usage(deps, id)?),
        QueryMsg::YieldFarmingInfo { id } => to_json_binary(&query_yield_farming_info(deps, id)?),
        QueryMsg::DataFeedIntegration { id } => to_json_binary(&query_data_feed_integration(deps, id)?),
        QueryMsg::DiscountCriteria { addr } => to_json_binary(&query_discount_criteria(deps, addr)?),


    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

pub fn query_options(deps: Deps)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> =OPTION_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let resp =options?;
    Ok(resp)
}

pub fn query_options_page(deps: Deps,key: u64,amount:u64)->StdResult<OptionsResponse>{
    let limit = amount as usize;
    let options:StdResult<Vec<_>> =OPTION_LIST.range(deps.storage, Some(Bound::exclusive(key)), None, Order::Ascending).take(limit).collect();
    let resp =options?;
    Ok(resp)
}

pub fn query_option_by_id(deps: Deps,id: u64)->StdResult<GetOptionByIdResponse>{
    let option = OPTION_LIST.load(deps.storage, id);
    Ok(option?)
}

pub fn query_market_options(deps: Deps)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> =MARKET_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let resp =options?;
    Ok(resp)

}

pub fn query_market_options_page(deps: Deps,key: u64,amount:u64)->StdResult<OptionsResponse>{
    let limit = amount as usize;
    let options:StdResult<Vec<_>> =MARKET_LIST.range(deps.storage, Some(Bound::exclusive(key)), None, Order::Ascending).take(limit).collect();
    let resp =options?;
    Ok(resp)
}

fn query_owner_options(deps: Deps,addr: String)->StdResult<OptionsResponse>{
    let key = deps.api.addr_validate(&addr)?;
    let options:StdResult<Vec<_>> = OWNER_LIST.prefix(key).range(deps.storage, None, None, Order::Ascending).collect();
    let resp =options?;
    Ok(resp)
}

fn query_creator_options(deps: Deps,addr: String)->StdResult<OptionsResponse>{
    let key = deps.api.addr_validate(&addr)?;
    let options:StdResult<Vec<_>> = CREATOR_LIST.prefix(key).range(deps.storage, None, None, Order::Ascending).collect();
    let resp =options?;
    Ok(resp)
}

fn query_bid_or_offer(deps: Deps, id: u64) -> StdResult<BidOrOfferResponse> {
    let option = OPTION_LIST.load(deps.storage, id)?;

    if let Some(bidder) = option.highest_bidder {
        let response = BidOrOfferResponse { 
            bidder: Some(bidder),
            offeror: None,
         };
        Ok(response)
    } else if let Some(offeror) = option.best_offer {
        let response = BidOrOfferResponse { 
            bidder: None,
            offeror: Some(offeror), 
        };
        Ok(response)
    } else {
        Err(cosmwasm_std::StdError::generic_err("No bid or offer found"))
    }
}

fn query_acceptance_status(deps: Deps, id: u64) -> StdResult<AcceptanceStatusResponse> {
    let option = OPTION_LIST.load(deps.storage, id)?;

    if let Some(_) = option.highest_bidder {
        Ok(AcceptanceStatusResponse::BidAccepted)
    } else if let Some(_) = option.best_offer {
        Ok(AcceptanceStatusResponse::OfferAccepted)
    } else {
        Ok(AcceptanceStatusResponse::NoBidOrOffer)
    }
}

fn query_bid_history(deps: Deps, option_id: u64) -> StdResult<Option<Vec<Bid>>> {
  
    let option = OPTION_LIST.load(deps.storage, option_id)?;  
    Ok(option.bid_history)
    
  }

fn query_bid_amount(deps: Deps, id: u64) -> StdResult<Vec<Coin>>{
    let option = OPTION_LIST.load(deps.storage, id)?;
    Ok(option.counter_offer)
    
}

fn query_offer_amount(deps: Deps, id: u64) -> StdResult<Vec<Coin>> {
    let option = OPTION_LIST.load(deps.storage, id)?;
    Ok(option.counter_offer)
    
}

fn query_latest_price(deps: Deps, oracle: String) -> StdResult<Vec<Coin>> {
    // Assuming oracle is validated using addr_validate function
    let oracle_address = deps.api.addr_validate(&oracle)?;

    // Use the validated oracle address to fetch the latest price from the oracle contract
      let latest_price: Vec<Coin> = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: oracle_address.to_string(),
        msg: to_json_binary(&QueryMsg::LatestPrice {oracle: oracle_address.to_string()})?.into(), 
    }))?;

    Ok(latest_price)
}

fn query_option_status(deps: Deps, id: u64) -> StdResult<OptionStatusResponse> {
    let option = OPTION_LIST.load(deps.storage, id)?;
    let response = OptionStatusResponse { status: option.status };
    Ok(response)
}

fn query_partial_execution(deps: Deps, id: u64) -> StdResult<PartialExecutionResponse> {
    let _option = OPTION_LIST.load(deps.storage, id)?;
    let response = PartialExecutionResponse {
        collateral_amount: Uint128::new(0),//for now replace after implementation with //option.collateral.amount,
        counter_offer_amount: Uint128::new(0), //for now replace after implementation with //option.counter_offer.amount,
    };
    Ok(response)
}

fn query_market_option_price(deps: Deps, id: u64) -> StdResult<MarketOptionPriceResponse> {
    let option = MARKET_LIST.load(deps.storage, id)?;
    let response = MarketOptionPriceResponse {
        price: option.price.clone(),
    };
    Ok(response)
}

// Query to get option exercise conditions
fn query_option_exercise_conditions(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //when implmemented
    //Ok(option.exercise_conditions.unwrap())
   
    // For now, return an empty vector
   Ok(Vec::new())
}

// Query to get option parameters
fn query_option_parameters(deps: Deps, id: u64) -> StdResult<HashMap<String, String>> {
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.parameters.unwrap())
    // For now, return an empty vector
    Ok(HashMap::new())
}

// Query to get option expiry notification details
fn query_option_expiry_notification(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get option expiry notification details (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.notification_details.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

// Query to get historical data of an option
fn query_option_history(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get historical data (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.history.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

// Query to get risk metrics for an option
fn query_option_risk_metrics(_deps: Deps, _id: u64) -> StdResult<HashMap<String, Decimal>> {
    // Placeholder logic to calculate risk metrics (replace with actual logic)
    let mut risk_metrics = HashMap::new();
    risk_metrics.insert("delta".to_string(), Decimal::zero());
    risk_metrics.insert("gamma".to_string(), Decimal::zero());
    risk_metrics.insert("theta".to_string(), Decimal::zero());
    // Add more risk metrics calculations as needed

    //Ok(risk_metrics)
    // For now, return an empty vector
    Ok(HashMap::new())
}

// Query to get option pool liquidity details
fn query_liquidity_provided(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get option pool liquidity details (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.liquidity_provided.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

// Query to get option pool liquidity details
fn query_liquidity_withdrawn(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get option pool liquidity details (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.liquidity_withdrawn.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

// Query to get option governance proposal details
fn query_option_governance_proposal(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get option governance proposal details (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.governance_proposal_details.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

// Query to get option collateral usage details
fn query_option_collateral_usage(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get option collateral usage details (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.collateral_usage_details.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

// Query to get AMM pool details for an option
fn query_amm_pool_details(deps: Deps, id: u64) -> StdResult<Vec<String>> {
    // Placeholder logic to get AMM pool details (replace with actual logic)
    let _option = OPTION_LIST.load(deps.storage, id)?;
    //Ok(option.amm_pool_details.unwrap())
    // For now, return an empty vector
    Ok(Vec::new())
}

pub fn query_collateral_usage(deps: Deps, id: u64) -> StdResult<CollateralUsageResponse> {
    // Placeholder logic for querying collateral usage
    // Replace with the actual logic for retrieving information about the specified option's use as collateral
    let _option = OPTION_LIST.load(deps.storage, id)?;
    let collateral_usage_info: CollateralUsageInfo = CollateralUsageInfo {
        in_use: false, // Replace with actual logic
        used_amount: Uint128::zero(), // Replace with actual logic
    };
    Ok(CollateralUsageResponse {
        collateral_usage_info,
    })
}

pub fn query_yield_farming_info(deps: Deps, id: u64) -> StdResult<YieldFarmingInfoResponse> {
    // Placeholder logic for querying yield farming info
    // Replace with the actual logic for retrieving information about the specified option's participation in yield farming
    let _option = OPTION_LIST.load(deps.storage, id)?;
    let yield_farming_info: YieldFarmingInfo = YieldFarmingInfo {
        is_wrapped: false, // Replace with actual logic
        wrapped_amount: Uint128::zero(), // Replace with actual logic
    };
    Ok(YieldFarmingInfoResponse {
        yield_farming_info,
    })
}

pub fn query_data_feed_integration(deps: Deps, id: u64) -> StdResult<DataFeedIntegrationResponse> {
    // Placeholder logic for querying data feed integration
    // Replace with the actual logic for retrieving information about the specified option's data feed integration
    let _option = OPTION_LIST.load(deps.storage, id)?;
    let data_feed_integration_info: DataFeedIntegrationInfo = DataFeedIntegrationInfo {
        is_integrated: false, // Replace with actual logic
        data_feed_url: String::new(), // Replace with actual logic
    };
    Ok(DataFeedIntegrationResponse {
        data_feed_integration_info,
    })
}

pub fn query_discount_criteria(deps: Deps, addr: Addr) -> StdResult<DiscountCriteriaResponse> {
    // Placeholder logic for querying discount criteria
    // Replace with the actual logic for retrieving discount criteria set by the specified user
    let discount_criteria = DISCOUNTS.load(deps.storage, &addr)?;
    
    let addr_ref = &addr;
    let addr_str = addr_ref.to_string();
    let criteria = discount_criteria.criteria.get(&addr_str).unwrap().clone();

    let user_criteria = hashmap! {
        addr.to_string() => criteria  
      };
      
      let _response = DiscountCriteriaResponse {
        discount_criteria: DiscountCriteria { criteria: user_criteria.clone() }
      };

    Ok(DiscountCriteriaResponse {
        discount_criteria: DiscountCriteria { criteria: user_criteria.clone() },
    })
}

/*
pub fn query_locked_tokens(deps: Deps, addr: Addr) -> Result<LockedTokensResponse, ContractError> {
    // Placeholder logic for querying locked tokens
    // Replace with the actual logic for retrieving information about the specified user's locked tokens
    let lock = LOCKS.load(deps.storage, &addr)?;
    Ok(LockedTokensResponse {
        locked_tokens: lock.locked_amount,
        expiration: lock.expiration,
    })
}

fn query_bid_history(deps: Deps, option_id: u64) -> Result<Vec<Bid>, ContractError> {
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    Ok(option.bid_history)
}

  pub fn query_liquidity_levels(deps: Deps, option_id: u64) -> Result<LiquidityProvided, ContractError> {
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    Ok(option.liquidity_levels)
}
  
  pub fn query_offer_history(deps: Deps, option_id: u64) -> Result<Vec<Offer>, ContractError> {
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    Ok(option.offer_history)
}
*/