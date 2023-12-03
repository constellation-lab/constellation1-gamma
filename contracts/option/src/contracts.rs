use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin,Timestamp,Order,Binary};
use cw_storage_plus::Bound;
use std::collections::HashMap;
use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, GovernanceMsg, CollateralMsg, YieldFarmingMsg}; 
use crate::msg::{AmmMsg, DataFeedMsg, OptionsResponse, GetOptionByIdResponse};
use crate::state::{State, CONFIG, Data,OPTION_LIST,CREATOR_LIST,OWNER_LIST,MARKET_LIST, DISCOUNTS};
use crate::state::{Bid, BidOrOfferResponse, AcceptanceStatusResponse, OptionStatus, DiscountCriteria};
use crate::state::{
    OptionStatusResponse, PartialExecutionResponse, MarketOptionPriceResponse,
    CollateralUsageResponse, CollateralUsageInfo, YieldFarmingInfoResponse, YieldFarmingInfo,
    DataFeedIntegrationResponse, DataFeedIntegrationInfo, DiscountCriteriaResponse,
};
use cosmwasm_std::{attr, Attribute, Event, Uint128, Addr, QueryRequest, WasmQuery, Decimal, WasmMsg, CosmosMsg};
use cosmwasm_schema::{serde::{Serialize, Deserialize}};
use maplit::hashmap;
//use serde_json::to_string;

// like solidity's constractor func, just run once when the contract init.
#[entry_point]
#[allow(dead_code)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        creator: info.sender.clone(),
        total_options_num: 0,
        paused: false,
        oracle: None,
    };


    CONFIG.save(deps.storage, &state)?;

    Ok(Response::default())
}

//The transaction msg will first come here, and the fn will route them to solver
#[entry_point]
#[allow(dead_code)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create{counter_offer,time_stamp} => execute_create(deps, env, info, counter_offer,time_stamp),
        ExecuteMsg::AddToMarket{id, amount, denom} => execute_add_to_market(deps, env, info,id, amount,&denom),
        ExecuteMsg::Burn { id }=> execute_burn(deps, info,id),
        ExecuteMsg::RemoveFromMarket { id } => execute_remove_from_market(deps,info,id),
        ExecuteMsg::Claim { id } => execute_claim(deps,env,id),
        ExecuteMsg::Buy { id }=> execute_buy(deps, env, info, id),
        ExecuteMsg::UpdatePrice { id,price }=> execute_update_price(deps,env,info,id,price),
        ExecuteMsg::Transfer {id, to } => execute_transfer(deps, env, info,id, to),
        ExecuteMsg::Execute { id }=>execute_execute(deps, env, info, id),
        ExecuteMsg::PlaceBid {id, bid_amount} => execute_place_bid(deps, env, info, id, bid_amount),
        ExecuteMsg::PlaceOffer {id, offer_amount} => execute_place_offer(deps, env, info, id, offer_amount),
        ExecuteMsg::AcceptBidOrOffer { id } => execute_accept_bid_or_offer(deps, env, id),
        ExecuteMsg::ExecutePartial {id,fraction} => execute_partial(deps, env, info, id, fraction),
        ExecuteMsg::BuyFraction {id, fraction} => execute_buy_fraction(deps, env, info, id, fraction),
        ExecuteMsg::WithdrawCollateral { id } => execute_withdraw_collateral(deps, env, info, id),
        ExecuteMsg::ExtendExpiration {id, new_expiration} => execute_extend_expiration(deps, env, info, id, new_expiration),
        ExecuteMsg::Pause {} => execute_pause(deps, info),
        ExecuteMsg::Unpause {} => execute_unpause(deps, info),  
        ExecuteMsg::AddOracle {oracle} => execute_add_oracle(deps, info, oracle),
        ExecuteMsg::UpdatePriceOracle {id, price} => execute_update_price_oracle(deps, env, info, id, price),
        ExecuteMsg::SetOptionExerciseConditions { id, exercise_conditions } => execute_set_option_exercise_conditions(deps, info, id, exercise_conditions),
        ExecuteMsg::SetOptionParameters { id, parameters } => execute_set_option_parameters(deps, info, id, parameters),
        ExecuteMsg::OptionExpiryNotification { id, notification_period } => execute_notify_option_expiry(deps, env, id, notification_period),
        ExecuteMsg::WrapOptionForYieldFarming { option_id, amount } => execute_wrap_option_for_yield_farming(deps, env, info, option_id, amount),
        ExecuteMsg::GetOptionHistory { id } => {
            let history = execute_get_option_history(deps, id)?;
            let history_json = serde_json::to_string(&history)?;   
            Ok(Response::new().add_attribute("history", history_json))},
        ExecuteMsg::CalculateOptionRiskMetrics { id } => {
            let metrics = execute_calculate_option_risk_metrics(deps, id)?;
            let metrics_json = serde_json::to_string(&metrics)?;
            Ok(Response::new().add_attribute("metrics", metrics_json))},
        ExecuteMsg::ProvideLiquidity { id, amounts } => execute_provide_liquidity(deps, env, info, id, amounts),
        ExecuteMsg::WithdrawLiquidity { id, amounts } => execute_withdraw_liquidity(deps, env, info, id, amounts),
        ExecuteMsg::VoteOnGovernanceProposal { proposal_id, vote } => execute_vote_on_governance_proposal(deps, env, info, proposal_id, vote),
        ExecuteMsg::UseOptionAsCollateral { option_id, amounts } => execute_use_option_as_collateral(deps, env, info, option_id, amounts),   
        ExecuteMsg::CreateAmmPool { option_id } => execute_create_amm_pool(deps, env, info, option_id),
        ExecuteMsg::TradeOnAmm {pool_id, amounts} => execute_trade_on_amm(deps, env, info, pool_id, amounts),
        ExecuteMsg::IntegrateDataFeed {option_id, feed_url} => execute_integrate_market_data_feed(deps, env, info, option_id, feed_url),    
        //ExecuteMsg::LockTokens { amounts, duration } => execute_lock_tokens(deps, env, info, amounts, duration),
        ExecuteMsg::ReferUser { user } => execute_refer_user(deps, info, user),
        ExecuteMsg::SetDiscountCriteria { criteria } => execute_set_discount_criteria(deps, info, criteria),  
    }
}

pub fn execute_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id:u64,

)-> Result<Response,ContractError>{
    let option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    //validate time and owner
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }
    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    if info.funds != option.counter_offer {
        return Err(ContractError::CounterOfferMismatch {
            offer: info.funds,
            counter_offer: option.counter_offer,
        });
    }
    let mut res: Response = Response::new();
    res = res.add_message(BankMsg::Send {
        to_address: option.creator.to_string(),
        amount: option.counter_offer,
    });
    res = res.add_message(BankMsg::Send {
        to_address: option.owner.to_string(),
        amount: option.collateral,
    });

    // Emit the Option executed event
    ConstellationDerivativeEvent::emit_execute_option(deps.as_ref(), id)?;

    OPTION_LIST.remove(deps.storage, id);
    OWNER_LIST.remove(deps.storage,(option.owner,id));
    CREATOR_LIST.remove(deps.storage,(option.creator,id));
    if MARKET_LIST.has(deps.storage, id){
        MARKET_LIST.remove(deps.storage, id);
    }
    res = res.add_attribute("action", "execute");
    Ok(res)
}

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id:u64,
    recipient: String,
) -> Result<Response, ContractError> {
    // ensure msg sender is the owner
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    //validate time and owner
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }
    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    //rm old owner
    OWNER_LIST.remove(deps.storage, (option.owner.clone(),id));

    //update option
    option.owner = deps.api.addr_validate(&recipient)?;

    if option.onsale == true{
        MARKET_LIST.remove(deps.storage, id);
        option.onsale = false;
        option.price = Vec::new();
    }

     // Emit the Option transferred event
     ConstellationDerivativeEvent::emit_option_transferred(deps.as_ref(), id, info.sender.to_string())?;

    OPTION_LIST.save(deps.storage,id , &option)?;
    CREATOR_LIST.save(deps.storage, (info.sender.clone(),id), &option)?;
    OWNER_LIST.save(deps.storage,(option.owner.clone(),id), &option)?;

    // set new owner on state
    
    let res: Response =
        Response::new().add_attributes([("action", "transfer"), ("owner", recipient.as_str())]);
    Ok(res)
}

pub fn execute_update_price(
    deps: DepsMut,
    env:Env,
    info: MessageInfo,
    id: u64,
    price: Vec<Coin>,
)->Result<Response,ContractError>{
    let mut option = match MARKET_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(_) => return Err(ContractError::OptionCanotFindInTheMarket{}),
    };

    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    if info.sender != option.owner{
        return Err(ContractError::Unauthorized {})
    }
    option.price = price;

    // Emit the price updated event
    ConstellationDerivativeEvent::emit_option_price_updated(deps.as_ref(), id)?;

    OPTION_LIST.save(deps.storage,id , &option)?;
    CREATOR_LIST.save(deps.storage, (info.sender.clone(),id), &option)?;
    OWNER_LIST.save(deps.storage,(info.sender.clone(),id), &option)?;
    MARKET_LIST.save(deps.storage,id , &option)?;
    let res: Response =
         Response::new().add_attributes([("action", "update price")]);

    Ok(res)
}

pub fn execute_buy(
    deps: DepsMut,
    env:Env,
    info: MessageInfo,
    id: u64,
)->Result<Response,ContractError>{
    let mut option = match MARKET_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(_) => return Err(ContractError::OptionCanotFindInTheMarket{}),
    };
    //validate is expires and pay enough token, and is expired
    if info.funds != option.price {
        return Err(ContractError::PriceMismatch {
            offer: info.funds,
            price: option.price,
        });
    }
    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    //send the token buyer paid to the owner
    let mut res: Response = Response::new().add_message(BankMsg::Send { to_address: option.owner.to_string(), amount: info.funds });
    //update stoge
    let old_owner = option.owner.clone();
    option.owner = info.sender.clone();
    option.price = Vec::new();
    option.onsale = false;

    // Emit the option bought event
    ConstellationDerivativeEvent::emit_option_bought(deps.as_ref(), id)?;

    MARKET_LIST.remove(deps.storage, id);
    OPTION_LIST.save(deps.storage, id, &option)?;
    OWNER_LIST.remove(deps.storage, (old_owner,id));
    OWNER_LIST.save(deps.storage, (option.owner.clone(),id), &option)?;
    CREATOR_LIST.save(deps.storage, (option.creator.clone(),id), &option)?;

    let owner = info.sender.clone().to_string();
    res = res.add_attributes([("action", "buy"), ("owner", &owner)]);
    Ok(res)
}

//claim expired option, remove the option and payback the collateral to the creator
pub fn execute_claim(
    deps:DepsMut,
    env: Env,
    id: u64,
)->Result<Response,ContractError>{
    let option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    if option.expires > env.block.time{
        return Err(ContractError::OptionNotExpired { expires: option.expires });
    }
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send { to_address: option.creator.to_string(), amount: option.collateral}).add_attribute("action", "claim");

    // Emit the event
    ConstellationDerivativeEvent::emit_option_claimed(deps.as_ref(), id)?;

    OPTION_LIST.remove(deps.storage, id);
    OWNER_LIST.remove(deps.storage,(option.owner,id));
    CREATOR_LIST.remove(deps.storage,(option.creator,id));
    if MARKET_LIST.has(deps.storage, id){
        MARKET_LIST.remove(deps.storage, id);
    }
    res = res.add_attribute("action", "claim");
    Ok(res)
}


pub fn  execute_create(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    counter_offer: Vec<Coin>,
    expires_time: u64,
)->Result<Response, ContractError>{
    let expires = Timestamp::from_seconds(expires_time);
    //validate the expires time
    if env.block.time > expires{
        return Err(ContractError::OptionExpired { expired: expires });
    }
    //save the state to Optionlist,Createor Option,Owner Option
    let new_data:Data = Data { 
        creator: info.sender.clone(), 
        owner: info.sender.clone(), 
        collateral: info.funds, 
        counter_offer: counter_offer, 
        onsale: false, 
        expires:expires ,
        price: Vec::new(), 
        highest_bidder: None, //highest_bidder: Some(Addr::unchecked("bidder_addr")), 
        best_offer: None,   //best_offer: Some(Addr::unchecked("offer_addr")),
        bid_history: None, 
        status: OptionStatus::Active,
        parameters: None,
        exercise_conditions: None,
        history: Vec::new(),
        risk_metrics: None,
        pool_share: Uint128::new(0), 
    };
    let mut state = CONFIG.load(deps.storage)?;

     //save the key id to the own and creator's list: key = id = option_id
     let key: u64 = state.total_options_num;
  
    //OPTION_LIST.save(deps.storage, key, &new_data);
    OPTION_LIST.save(deps.storage,state.total_options_num , &new_data)?;
    CREATOR_LIST.save(deps.storage, (info.sender.clone(),key), &new_data)?;
    OWNER_LIST.save(deps.storage,(info.sender.clone(),key), &new_data)?;

    //save the total_option+1
    state.total_options_num=key + 1;
    CONFIG.save(deps.storage, &state)?;

    // Emit the event
    ConstellationDerivativeEvent::emit_option_created(deps.as_ref(), key)?;

    let res: Response =
        Response::new().add_attributes([("action", "create option"), ("id", &key.to_string())]);
    Ok(res)
}

//list the option on the market
pub fn execute_add_to_market(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
    amount:u128,
    denom: &str,
)->Result<Response,ContractError>{
    //load the option data
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    //validate the send is the owner
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }
    //validate is the option expired
    if env.block.time > option.expires{
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    //set the option data to on sale and set price
    option.onsale = true;
    option.price = vec![(Coin::new(amount, denom))];

    ConstellationDerivativeEvent::emit_option_added_to_market(deps.as_ref(), id)?;

    //save the change 
    OPTION_LIST.save(deps.storage, id, &option)?;
    OWNER_LIST.save(deps.storage, (option.owner.clone(),id), &option)?;
    CREATOR_LIST.save(deps.storage, (option.creator.clone(),id), &option)?;
    MARKET_LIST.save(deps.storage,id , &option)?;
    let res: Response =
    Response::new().add_attributes([("action", "add to market"), ("id", &id.to_string()), ("price",&amount.to_string())]);
    Ok(res)
}

//burn the option and send the collateral  to the creator
pub fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
)->Result<Response, ContractError>{
    //load the option data
    let option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    //validate the sender is the owner
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send {
        to_address: option.creator.to_string(),
        amount: option.collateral,
    });

    ConstellationDerivativeEvent::emit_option_burned(deps.as_ref(), id, info.sender.to_string() )?;

    OPTION_LIST.remove(deps.storage, id);
    OWNER_LIST.remove(deps.storage,(option.owner,id));
    CREATOR_LIST.remove(deps.storage,(option.creator,id));
    if MARKET_LIST.has(deps.storage, id){
        MARKET_LIST.remove(deps.storage, id);
    }
    res = res.add_attribute("action", "burn");
    Ok(res)
}

//remove the option from market list
pub fn execute_remove_from_market(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
)-> Result<Response, ContractError>{
    //validate the sender is the onwer
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }
    option.onsale = false;
    option.price = Vec::new();

    ConstellationDerivativeEvent::emit_option_removed_from_market(deps.as_ref(), id)?;

    OPTION_LIST.save(deps.storage, id, &option)?;
    MARKET_LIST.remove(deps.storage, id);
    OWNER_LIST.save(deps.storage, (option.owner.clone(),id), &option)?;
    CREATOR_LIST.save(deps.storage, (option.creator.clone(),id), &option)?;

    let res = Response::new().add_attribute("action", "remove form market");
    Ok(res)
}

//function additions for partial execution, fractional trading, bids, etc
pub fn execute_place_bid(deps: DepsMut, env: Env, info: MessageInfo, id: u64, bid_amount: Vec<Coin>) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let mut option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };
    // Validate the bid
    if info.funds != bid_amount {
        return Err(ContractError::BidAmountMismatch {
            bid_amount,
            expected_bid_amount: option.counter_offer,
        });
    }
     //Validate the time
    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    // Emit the event  
    ConstellationDerivativeEvent::emit_bid_placed(deps.as_ref(), id)?;
    // Update the state with the new bid
    option.counter_offer = bid_amount;
    option.highest_bidder = Some(info.sender.clone());
    option.best_offer = None; // set this to None when placing a bid

    OPTION_LIST.save(deps.storage, id, &option)?;

    let res: Response = Response::new().add_attributes([("action", "place_bid")]);
    Ok(res)
}

pub fn execute_place_offer(deps: DepsMut, env: Env, info: MessageInfo, id: u64, offer_amount: Vec<Coin>) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let mut option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };
    // Validate the offer
    if info.funds != offer_amount {
        return Err(ContractError::OfferAmountMismatch {
            offer_amount,
            expected_offer_amount: option.counter_offer,
        });
    }
    // Validate the time
    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    // Update the state with the new offer
    option.counter_offer = offer_amount;
    option.best_offer = Some(info.sender.clone());
    option.highest_bidder = None; //set this to None when placing an offer
    
    // Emit the event
    ConstellationDerivativeEvent::emit_offer_placed(deps.as_ref(), id)?;
    OPTION_LIST.save(deps.storage, id, &option)?;
    let res: Response = Response::new().add_attributes([("action", "place_offer")]);
    Ok(res)
}

pub fn execute_accept_bid_or_offer(deps: DepsMut, env: Env, id: u64) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };

    // Validate the time
    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    // Check if there is a bid or offer to accept
    if let Some(bidder) = option.highest_bidder.clone() {
        // Accept the highest bid
        let mut res: Response = Response::new().add_message(BankMsg::Send {
            to_address: bidder.clone().to_string(),
            amount: option.counter_offer,
        });
        // Transfer the collateral to the owner
        res = res.add_message(BankMsg::Send {
            to_address: option.owner.clone().to_string(),
            amount: option.collateral,
        });

        ConstellationDerivativeEvent::emit_bid_or_offer_accepted(deps.as_ref(), id)?;

        // Remove the option from storage
        OPTION_LIST.remove(deps.storage, id);
        OWNER_LIST.remove(deps.storage, (option.owner.clone(), id));
        CREATOR_LIST.remove(deps.storage, (option.creator.clone(), id));

        res = res.add_attribute("action", "accept_bid");
        Ok(res)
    } else if let Some(offeror) = option.best_offer.clone() {
        // Accept the best offer
        let mut res: Response = Response::new().add_message(BankMsg::Send {
            to_address: offeror.clone().to_string(),
            amount: option.counter_offer,
        });

        // Transfer the collateral to the owner
        res = res.add_message(BankMsg::Send {
            to_address: option.owner.clone().to_string(),
            amount: option.collateral,
        });

        // Remove the option from storage
        OPTION_LIST.remove(deps.storage, id);
        OWNER_LIST.remove(deps.storage, (option.owner.clone(), id));
        CREATOR_LIST.remove(deps.storage, (option.creator.clone(), id));

        res = res.add_attribute("action", "accept_offer");
        Ok(res)
    } else {
        Err(ContractError::NoBidOrOffer {})
    }
}

pub fn execute_partial(deps: DepsMut, env: Env, info: MessageInfo, id: u64, fraction: f64) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let mut option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };

        // Validate the time -reconfirm if this is needed
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

    // Validate the owner
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Validate the option's status
    if option.status != OptionStatus::Active {
        return Err(ContractError::InvalidOptionStatus {});
    }

    // Validate the fraction
    if fraction <= 0.0 || fraction > 1.0 {
        return Err(ContractError::InvalidFraction {});
    }

    // Calculate the partial amounts
   
    let partial_collateral = (fraction * option.collateral[0].amount.u128() as f64) as u128;
    let partial_counter_offer = (fraction * option.counter_offer[0].amount.u128() as f64) as u128;
    
    // Transfer partial collateral and counter offer to the owner
    let res: Response = Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: option.collateral[0].denom.clone(),
                amount: Uint128::new(partial_collateral),
            }],
        })
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: option.counter_offer[0].denom.clone(),
                amount: Uint128::new(partial_counter_offer),
            }],
        });

    // Update the option state
    let partial_collateral = Uint128::from(partial_collateral);
    let partial_counter_offer = Uint128::from(partial_counter_offer);

    option.collateral[0].amount = option.collateral[0].amount - partial_collateral;
    option.counter_offer[0].amount = option.counter_offer[0].amount - partial_counter_offer;

    
    ConstellationDerivativeEvent::emit_partial_execution(deps.as_ref(), id)?;
    
    OPTION_LIST.save(deps.storage, id, &option)?;

    Ok(res)
}

pub fn execute_buy_fraction(deps: DepsMut, env: Env, info: MessageInfo, id: u64, fraction: f64) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let mut option = match MARKET_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionCanotFindInTheMarket {}),
    };

    // Validate the time
    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    // Validate the fraction
    if fraction <= 0.0 || fraction > 1.0 {
        return Err(ContractError::InvalidFraction {});
    }

    // Calculate the fractional amount to be bought
   // fraction_amount = (option.price[0].amount.u128() * (fraction * 1000000.0) as u128) / 1000000;
    let fraction_amount = (option.price[0].amount.u128() as f64 * fraction) as u128;

    // Validate if the buyer has enough funds
    if info.funds[0].amount < Uint128::from(fraction_amount) {
        return Err(ContractError::InsufficientFunds {});
    }

    // Transfer funds to the owner
    let res: Response = Response::new().add_message(BankMsg::Send {
        to_address: option.owner.to_string(),
        amount: vec![Coin {
            denom: option.price[0].denom.clone(),
            amount: Uint128::new(fraction_amount),
        }],
    });

    // Update the state
    option.price[0].amount = option.price[0].amount - Uint128::from(fraction_amount);

    ConstellationDerivativeEvent::emit_fractional_buy(deps.as_ref(), id, fraction.to_string())?;
    

    MARKET_LIST.save(deps.storage, id, &option)?;

    Ok(res)
}

pub fn execute_withdraw_collateral(deps: DepsMut, env: Env,  info: MessageInfo, id: u64) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };

    // Validate the sender
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Validate the time
    if env.block.time < option.expires {
        return Err(ContractError::WithdrawBeforeExpiration {});
    }

    // Transfer the collateral to the owner
    let res: Response = Response::new().add_message(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: option.collateral,
    });

    ConstellationDerivativeEvent::emit_withdraw_collateral(deps.as_ref(), info.sender.clone(), id)?;

    // Remove the option from storage
    OPTION_LIST.remove(deps.storage, id);
    OWNER_LIST.remove(deps.storage, (option.owner, id));
    CREATOR_LIST.remove(deps.storage, (option.creator, id));

    Ok(res)
}

pub fn execute_extend_expiration(deps: DepsMut, env: Env, info: MessageInfo, id: u64, new_expiration: u64) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let mut option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };

    // Validate the sender
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Validate the time and new expiration
    if env.block.time >= option.expires 
   || Timestamp::from_seconds(new_expiration) <= env.block.time 
    {
        return Err(ContractError::InvalidExpiration {});
    }

    // Update the state with the new expiration time
    option.expires = Timestamp::from_seconds(new_expiration);

    ConstellationDerivativeEvent::emit_expiration_extended(deps.as_ref(), id, new_expiration)?;
    

    OPTION_LIST.save(deps.storage, id, &option)?;

    let res: Response = Response::new().add_attributes([("action", "extend_expiration")]);
    Ok(res)
}


pub fn execute_pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate the sender
    if info.sender != CONFIG.load(deps.storage)?.creator {
        return Err(ContractError::Unauthorized {});
    }

    // Update the paused state
    CONFIG.update(deps.storage, |mut state| {
        state.paused = true;
        Ok::<_, cosmwasm_std::StdError>(state)
    })?;

    ConstellationDerivativeEvent::emit_contract_paused(deps.as_ref())?;
    

    let res: Response = Response::new().add_attributes([("action", "pause")]);
    
    Ok(res)
}

pub fn execute_unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate the sender
    if info.sender != CONFIG.load(deps.storage)?.creator {
        return Err(ContractError::Unauthorized {});
    }

    // Update the paused state
    CONFIG.update(deps.storage, |mut state| {
        state.paused = false;
        Ok::<_, cosmwasm_std::StdError>(state)
    })?;

    ConstellationDerivativeEvent::emit_contract_unpaused(deps.as_ref())?;

    let res: Response = Response::new().add_attributes([("action", "unpause")]);

    Ok(res)
    }

    pub fn execute_add_oracle(deps: DepsMut, info: MessageInfo, oracle: Addr) -> Result<Response, ContractError> {
        // Validate the sender
        if info.sender != CONFIG.load(deps.storage)?.creator {
            return Err(ContractError::Unauthorized {});
        }
    
        // Add the oracle address to the state
        CONFIG.update(deps.storage, |mut state| {
            state.oracle = Some(oracle.clone());
            Ok::<_, cosmwasm_std::StdError>(state)
        })?;
    
        ConstellationDerivativeEvent::emit_oracle_added(deps.as_ref(),  info.sender.clone(), oracle.clone())?;

        let res: Response = Response::new().add_attributes([("action", "add_oracle"), ("oracle", oracle.as_str())]);
        
        Ok(res)
    }

    pub fn  execute_update_price_oracle( deps: DepsMut, env: Env, _info: MessageInfo, id: u64, price: Vec<Coin>) -> Result<Response, ContractError> {
        // Load the existing option from storage
        let mut option = match MARKET_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(_) => return Err(ContractError::OptionCanotFindInTheMarket {}),
        };

        // Validate the time
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Validate that an oracle is set
        let oracle_address = match CONFIG.load(deps.storage) {
            Ok(state) => match state.oracle {
                Some(oracle) => oracle,
                None => return Err(ContractError::OracleNotSet {}),
            },
            Err(_) => return Err(ContractError::ConfigNotFound {}),
        };

        // Fetch the latest price from the oracle
        let latest_price: Vec<Coin> = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: oracle_address.to_string(),
            msg: to_json_binary(&QueryMsg::LatestPrice {oracle: oracle_address.to_string()})?.into(), 
        }))?;

        // Update the option's price with the latest from the oracle
        option.price = latest_price.clone();

        ConstellationDerivativeEvent::emit_price_updated_with_oracle(deps.as_ref(), price)?;
        

        MARKET_LIST.save(deps.storage, id, &option)?;

        let res: Response = Response::new().add_attributes([("action", "update_price_oracle")]);
        Ok(res)
    }


    // Set conditions for option exercise, such as specific events or external data criteria.
    pub fn execute_set_option_exercise_conditions(
        deps: DepsMut,
        info: MessageInfo,
        id: u64,
        exercise_conditions: Vec<String>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let mut option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate the sender is the creator of the option
        if info.sender != option.creator {
            return Err(ContractError::Unauthorized {});
        }

        // Update the option exercise conditions
        option.exercise_conditions = Some(exercise_conditions.clone());

        ConstellationDerivativeEvent::emit_option_exercise_conditions_set(deps.as_ref(),  id, exercise_conditions)?;


        // Save the updated option
        OPTION_LIST.save(deps.storage, id, &option)?;

        // Return a success response
        Ok(Response::new().add_attribute("action", "set_option_exercise_conditions"))
    }

    // Allow the setting of additional option parameters like strike prices, exercise styles, and other contract-specific details.
    pub fn execute_set_option_parameters(
        deps: DepsMut,
        info: MessageInfo,
        id: u64,
        parameters: HashMap<String, String>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let mut option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate the sender is the creator of the option
        if info.sender != option.creator {
            return Err(ContractError::Unauthorized {});
        }

        // Update the option parameters
        option.parameters = Some(parameters.clone());

        ConstellationDerivativeEvent::emit_option_parameters_set(deps.as_ref(),  id, parameters)?;
        
        // Save the updated option
        OPTION_LIST.save(deps.storage, id, &option)?;

        // Return a success response
        Ok(Response::new().add_attribute("action", "set_option_parameters"))
    }

    // Notify option holders and the contract when an option is about to expire.
    pub fn execute_notify_option_expiry(
        deps: DepsMut,
        env: Env,
        id: u64,
        notification_period: u64,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate that the current time is within the notification period before expiry
        if option.expires.seconds() - env.block.time.seconds() <= notification_period {
            // Notify option holders
            // (This is a placeholder and should be replaced with the actual logic to notify option holders)

            ConstellationDerivativeEvent::emit_option_expired_notification(deps.as_ref(),  id, notification_period)?;
            // Construct a response with the notification information
            let res = Response::new().add_attribute(
                "action",
                "notify_option_expiry",
            );

            // Return the response
            Ok(res)
        } else {
            // If not within the notification period, return an error
            Err(ContractError::InvalidExpirationNotification {})
        }
    }


    // Get historical data of an option, such as previous prices, exercise events, and other relevant information.
    pub fn execute_get_option_history(deps: DepsMut, id: u64) -> Result<Vec<String>, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        ConstellationDerivativeEvent::emit_option_history(deps.as_ref(), id, Vec::<String>::new())?;

        // Return the historical data (This is a placeholder and should be replaced with actual historical data)
        Ok(option.history)
        
    }

    // Calculate risk metrics for an option, such as the delta, gamma, theta, and other parameters.
    pub fn execute_calculate_option_risk_metrics(deps: DepsMut, id: u64) -> Result<HashMap<String, Decimal>, ContractError> {
        // Load the option data
        let _option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Calculate risk metrics (This is a placeholder and should be replaced with actual calculations or info from optin above)
        let mut risk_metrics = HashMap::new();
        risk_metrics.insert("delta".to_string(), Decimal::zero());
        risk_metrics.insert("gamma".to_string(), Decimal::zero());
        risk_metrics.insert("theta".to_string(), Decimal::zero());
        // Add more risk metrics calculations as needed

        ConstellationDerivativeEvent::emit_option_risk_metrics_calculated(deps.as_ref(),  id, risk_metrics.clone())?;

        Ok(risk_metrics)
    }

    // Allow users to provide liquidity to the option pool, receiving LP tokens in return.
    pub fn execute_provide_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: u64,
        liquidity_amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate that the current time is before the option expiration
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Perform the liquidity provision logic (This is a placeholder and should be replaced with actual logic)
        // Assume LP tokens are minted and sent to the liquidity provider
        let mut res = Response::new();
        res = res.add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: liquidity_amount.clone(),
        });

        ConstellationDerivativeEvent::emit_liquidity_provided(deps.as_ref(), info.sender.to_string(), serde_json::to_string(&liquidity_amount).map_err(|_| ContractError::JsonSerializationError {})?)?;
        // Update the option state or pool balances as needed

        res = res.add_attribute("action", "provide_liquidity");
        Ok(res)
    }

    // Allow users to withdraw liquidity from the option pool, burning LP tokens.
    pub fn execute_withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: u64,
        liquidity_amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate that the current time is before the option expiration
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Perform the liquidity withdrawal logic (This is a placeholder and should be replaced with actual logic)
        // Assume LP tokens are burned and the corresponding assets are sent to the liquidity provider
        let mut res = Response::new();
        res = res.add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: liquidity_amount.clone(),
        });
        // Update the option state or pool balances as needed

        ConstellationDerivativeEvent::emit_liquidity_withdrawn(deps.as_ref(), info.sender.to_string(), serde_json::to_string(&liquidity_amount).map_err(|_| ContractError::JsonSerializationError {})?)?;

        res = res.add_attribute("action", "withdraw_liquidity");
        Ok(res)
    }

    // Allow users to vote on a governance proposal related to the option protocol.
    pub fn execute_vote_on_governance_proposal(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo, //may be used when governance proposal is fully implemented
        proposal_id: u64,
        vote: bool,
    ) -> Result<Response, ContractError> {
        // Placeholder logic for governance voting
        // Assume there is a separate governance contract, and this contract communicates with it.
        // This should be replaced with the actual logic for interacting with the governance module.

        // Validate that the current time is before the option expiration
        let option_id = proposal_id; // Assuming the proposal ID corresponds to an option ID
        let option = match OPTION_LIST.load(deps.storage, option_id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), proposal_id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Placeholder logic for voting (replace with actual governance contract interaction)
        // For illustration purposes, assume we send a message to a governance contract to register the vote.
        // This will likely involve interacting with the governance contract's vote method.

        ConstellationDerivativeEvent::emit_vote_on_governance_proposal(deps.as_ref(), proposal_id, vote)?;
        let governance_contract = Addr::unchecked("your_governance_contract_address"); // Replace with actual address
        let governance_msg = GovernanceMsg::Vote {
            proposal_id,
            vote,
        };
        let mut res = Response::new();
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: governance_contract.into(),
            msg: to_json_binary(&governance_msg)?,
            funds: vec![],
        }));

        res = res.add_attribute("action", "vote_on_governance_proposal");
        Ok(res)
    }

    // Allow users to use an option as collateral for other financial activities.
    pub fn execute_use_option_as_collateral(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo, //may be used during full implementation fo placeholder logic
        option_id: u64,
        amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Placeholder logic for using option as collateral
        // This should be replaced with the actual logic for interacting with other financial modules.

        // Validate that the current time is before the option expiration
        let option = match OPTION_LIST.load(deps.storage, option_id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Placeholder logic for using option as collateral (replace with actual logic)
        // This could involve sending a message to another module that supports using options as collateral.
        // For illustration purposes, assume we send a message to a hypothetical collateral module.

        ConstellationDerivativeEvent::emit_option_used_as_collateral(deps.as_ref(), option_id, amount.clone())?;

        let collateral_module = Addr::unchecked("your_collateral_module_address"); // Replace with actual address
        let collateral_msg = CollateralMsg::UseOptionAsCollateral {
            option_id,
            amount,
        };
        let mut res = Response::new();
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collateral_module.into(),
            msg: to_json_binary(&collateral_msg)?,
            funds: vec![],
        }));

        res = res.add_attribute("action", "use_option_as_collateral");
        Ok(res)
    }

    // Wrap an option to participate in a yield farming program.
    pub fn execute_wrap_option_for_yield_farming(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,  //may be used during full implementation fo placeholder logic
        option_id: u64,
        amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Placeholder logic for wrapping option for yield farming
        // This should be replaced with the actual logic for interacting with a yield farming module.

        // Validate that the current time is before the option expiration
        let option = match OPTION_LIST.load(deps.storage, option_id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Placeholder logic for wrapping option for yield farming (replace with actual logic)
        // This could involve sending a message to another module that supports wrapping options for yield farming.
        // For illustration purposes, assume we send a message
        ConstellationDerivativeEvent::emit_option_wrapped_for_yield_farming(deps.as_ref(), option_id, amount.clone())?;
        
        let yield_farming_module = Addr::unchecked("your_yield_farming_module_address"); // Replace with actual address
        let
        
        
        yield_farming_msg = YieldFarmingMsg::WrapOptionForYieldFarming {
            option_id,
            amount,
        };
        let mut res = Response::new();
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: yield_farming_module.into(),
            msg: to_json_binary(&yield_farming_msg)?,
            funds: vec![],
        }));

        res = res.add_attribute("action", "wrap_option_for_yield_farming");
        Ok(res)
    }



// Create an Automated Market Maker (AMM) pool for trading options.
pub fn execute_create_amm_pool(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo, //might use during full implementation
    option_id: u64,
) -> Result<Response, ContractError> {
    // Placeholder logic for creating AMM pool
    // This should be replaced with the actual logic for creating an AMM pool.

    // Validate that the current time is before the option expiration
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    ConstellationDerivativeEvent::emit_amm_pool_created(deps.as_ref(), option_id)?;

    // Placeholder logic for creating AMM pool (replace with actual logic)
    // This could involve sending a message to another module that supports AMM pool creation.
    // For illustration purposes, assume we send a message to a hypothetical AMM module.
    let amm_module = Addr::unchecked("your_amm_module_address"); // Replace with actual address
    let amm_msg = AmmMsg::CreatePool { option_id };
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: amm_module.into(),
        msg: to_json_binary(&amm_msg)?,
        funds: vec![],
    }));

    res.clone().add_attribute("action", "create_amm_pool");
    Ok(res)
}

// Trade options on an existing Automated Market Maker (AMM) pool.
pub fn execute_trade_on_amm(
    deps: DepsMut,
    _env: Env, //might use during full implementation
    _info: MessageInfo, //might use during full implementation
    pool_id: u64,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    // Placeholder logic for trading on AMM
    // This should be replaced with the actual logic for trading options on an AMM pool.

    // Placeholder logic for validating trade conditions (replace with actual validation logic)
    if amount.is_empty() {
        return Err(ContractError::InvalidTradeAmount {});
    }

    ConstellationDerivativeEvent::emit_option_traded_on_amm(deps.as_ref(), pool_id, amount.clone())?;

    // Placeholder logic for trading on AMM (replace with actual logic)
    // This could involve sending a message to another module that supports AMM trading.
    // For illustration purposes, assume we send a message to a hypothetical AMM module.
    let amm_module = Addr::unchecked("your_amm_module_address"); // Replace with actual address
    let amm_msg = AmmMsg::Trade { pool_id, amount };
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: amm_module.into(),
        msg: to_json_binary(&amm_msg)?,
        funds: vec![],
    }));

    res.clone().add_attribute("action", "trade_on_amm");
    Ok(res)
}

// Integrate market data feed for options pricing.
pub fn execute_integrate_market_data_feed(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo, //might use during full implementation
    option_id: u64,
    data_feed_url: String,
) -> Result<Response, ContractError> {
    // Placeholder logic for integrating market data feed
    // This should be replaced with the actual logic for integrating a market data feed.

    // Validate that the current time is before the option expiration
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    ConstellationDerivativeEvent::emit_market_data_feed_integrated(deps.as_ref(), option_id, data_feed_url.clone())?;

    // Placeholder logic for integrating market data feed (replace with actual logic)
    // This could involve sending a message to another module that supports data feed integration.
    // For illustration purposes, assume we send a message to a hypothetical data feed module.
    let data_feed_module = Addr::unchecked("your_data_feed_module_address"); // Replace with actual address
    let data_feed_msg = DataFeedMsg::Integrate { option_id, data_feed_url };
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: data_feed_module.into(),
        msg: to_json_binary(&data_feed_msg)?,
        funds: vec![],
    }));

    res.clone().add_attribute("action", "integrate_market_data_feed");
    Ok(res)
}

pub fn execute_refer_user(
    deps: DepsMut,
    info: MessageInfo,
    referred_user: Addr,
) -> Result<Response, ContractError> {
    // Placeholder logic for referral program
    // This should be replaced with the actual logic for tracking referrals and providing rewards.

    // Assume a referral reward structure
    let referral_reward = Coin {
        denom: "uusd".to_string(),
        amount: Uint128::new(100), // Adjust the reward amount as needed
    };

    ConstellationDerivativeEvent::emit_user_referred(deps.as_ref(), referred_user.clone())?;

    // Transfer referral reward to the referring user
    let res = Response::new();
    res.clone().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![referral_reward],
    }));

    // Add attributes for referral
    res.clone().add_attribute("action", "refer_user");
    res.clone().add_attribute("referring_user", info.sender);
    res.clone().add_attribute("referred_user", referred_user);

    Ok(res)
}


pub fn execute_set_discount_criteria(
    deps: DepsMut,
    info: MessageInfo,
    criteria: HashMap<String, String>,
) -> Result<Response, ContractError> {
    // Placeholder logic for setting discount criteria
    // This should be replaced with the actual logic for allowing users to set their own discount criteria.

    // Assume a discount criteria structure in the contract state

    let sender_raw = deps.api.addr_validate(&(&info.sender).to_string())?;
    let mut discounts = DISCOUNTS.load(deps.storage, &sender_raw)?;
    //discounts.insert
    discounts.criteria.insert(info.sender.clone().to_string(), criteria.clone());
    DISCOUNTS.save(deps.storage, &sender_raw, &discounts)?;

    ConstellationDerivativeEvent::emit_discount_criteria_set(deps.as_ref(), info.sender.clone(), criteria)?;

    // Add attributes for setting discount criteria
    let res = Response::new();
    res.clone().add_attribute("action", "set_discount_criteria");
    res.clone().add_attribute("user", info.sender);

    Ok(res)
}

/*
pub fn execute_create_token(
    deps: DepsMut, info: MessageInfo, counter_offer: Coin,time_stamp: Timestamp,) -> Result<Response, ContractError> {
    // Validate the sender
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Create a new option with token as collateral
    let new_option = Option {
        id: generate_option_id(deps.api, &info.sender, &time_stamp),
        creator: info.sender.clone(),
        owner: info.sender.clone(),
        collateral: counter_offer,
        counter_offer: Coin {
            denom: "uatom".to_string(),
            amount: "0".to_string(),
        },
        expires: time_stamp + config.option_duration,
        status: OptionStatus::Active,
        price: Vec::new(),
        onsale: false,
    };

    ConstellationDerivativeEvent::emit_token_option_created(deps.as_ref(), new_option.id.clone(), info.sender.clone())?;

    // Save the new option to storage
    OPTION_LIST.save(deps.storage, new_option.id.clone(), &new_option)?;

    let res: Response = Response::new().add_attributes([("action", "create_token")]);
    Ok(res)
}

// Sample Function to generate an option ID if needed
fn generate_option_id(api: &dyn Api, creator: &str, time_stamp: &u64) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(creator.as_bytes());
    bytes.extend_from_slice(&time_stamp.to_be_bytes());

    // You can customize this part based on your requirements
    let hash = api.sha256(&bytes);
    hex::encode(hash)
}
pub fn execute_lock_tokens(
    deps: DepsMut, env: Env, info: MessageInfo, amount: Vec<Coin>, lock_duration: u64,) -> Result<Response, ContractError> {
    // Placeholder logic for locking tokens
    // This should be replaced with the actual logic for locking tokens and providing exclusive access.
    // Assume a lock structure and add the user to the lock list
    let lock = Lock {
        expiration: env.block.time.seconds() + lock_duration, 
        user: info.sender.clone(),
        locked_amount: amount.clone(),
    };
    LOCKS.save(deps.storage, &info.sender, &lock)?;

    // Assume an ERC-20 token contract and transfer tokens to this contract
    let token_contract = Addr::unchecked("your_token_contract_address"); // Replace with actual address
    let lock_msg = Cw20ExecuteMsg::Transfer {
        recipient: env.contract.address.clone().into(),
        amount,
    };

    ConstellationDerivativeEvent::emit_tokens_locked(deps.as_ref(), amount, lock_duration)?;

    let mut res = Response::new();
    res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_contract.into(),
        msg: to_json_binary(&lock_msg)?,
        funds: vec![],
    }));

    // Add attributes for locking tokens
    res.add_attribute("action", "lock_tokens");
    res.add_attribute("user", info.sender);

    Ok(res)
}
//sample liquidity implementtion 
pub struct Liquidity {
  pub locked: Uint128,
  pub unlocked: Uint128,
}
pub fn total_liquidity(deps: Deps) -> StdResult<Liquidity> {
  let locked = LOCKED_COINS
    .range(deps.storage, None, None, Order::Ascending])
    .map(|item| item.amount)
    .sum();
  let unlocked = UNLOCKED_COINS
    .range(deps.storage, None, None, Order::Ascending])  
    .map(|item| item.amount)
    .sum();
  Ok(Liquidity{
    locked,
    unlocked    
  })  
}
//IMPLEMENTATION OF ONLY OWNER MODIFIER

// Modify trait definition with two generic arguments
pub trait Modify<T> {
    fn modify(&self, deps: DepsMut, env: Env, msg: T) -> Result<T, ContractError>;
}
// Implement Modify for OnlyOwner
impl<T> Modify<T> for OnlyOwner {
    fn modify(&self, deps: DepsMut, env: Env, msg: T) -> Result<T, ContractError> {
        let sender = env.message.sender;
        let config: Config = CONFIG.load(deps.storage)?;
        if sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(msg)
    }
}

EXAMPLE USE OF onlyOwner Modifier
pub fn set_discount_criteria(
    deps: DepsMut, env: Env, info: MessageInfo, criteria: HashMap<String, String>,) -> Result<Response, ContractError> {
    let only_owner = OnlyOwner {};
    only_owner.modify(deps.branch(), env.clone(), ())?;    // this Ensures only owner can modify
    // Rest of your existing logic for setting discount criteria...
    Ok(res)
}
*/

//Events implementtion
#[derive(Serialize, Deserialize)]
pub enum ConstellationDerivativeEvent {
    BidPlaced { bid_id: u64 },
    BidOrOfferAccepted { id: u64 },
    OptionCreated { id: u64 },
    OptionClaimed { id: u64 },
    OfferPlaced { id: u64 },
    PartialExecution{id: u64},
    FractionalBuy{id: u64, amount: String},
    WithdrawCollateral{sender: Addr, id: u64},
    ExtendExpiration{id: u64, new_expiration: u64},
    ContractPaused{},
    ContractUnpaused{},
    OracleAdded{sender: Addr, oracle: Addr},
    PriceUpdatedWithOracle{price: Vec<Coin>},
    OptionExerciseConditionsSet{id: u64, exercise_conditions: Vec<String>},
    OptionExpiredNotification { id: u64, notification_period: u64 },
    OptionParametersSet { id: u64, parameters: HashMap<String, String> },
    OptionHistory { id: u64, history: Vec<String> },
    OptionRiskMetricsCalculated { id: u64, risk_metrics: HashMap<String, Decimal> },
    LiquidityProvided { provider: String, amount: String },
    LiquidityWithdrawn { provider: String, amount: String },
    VoteOnGovernanceProposal { proposal_id: u64, vote: bool },
    OptionUsedAsCollateral { option_id: u64, amount: Vec<Coin> },
    OptionWrappedForYieldFarming { option_id: u64, amount: Vec<Coin> },
    AmmPoolCreated { option_id: u64 },
    OptionTradedOnAmm { pool_id: u64, amount: Vec<Coin> },
    MarketDataFeedIntegrated { option_id: u64, data_feed_url: String },
    TokensLocked { amount: Vec<Coin>, lock_duration: u64 },
    UserReferred { referred_user: Addr },
    DiscountCriteriaSet { user: Addr, criteria: HashMap<String, String> },
    OptionTransferred { id: u64, new_owner: String },
    OptionExecuted{id: u64},
    OptionExpired { id: u64 },
    PriceUpdated {id: u64, /*price: String*/},
    BuyOption{id: u64, /*price: String*/}, 
    OptionAddedToMarket {id: u64,/*amount: String,denom: String,*/},
    OptionBurned {id: u64,sender: String,/*creator: String,*/},
    OptionRemovedFromMarket {id: u64, /*sender: String,*/},
       // BidWithdrawn { bid_id: u64 },
}


impl ConstellationDerivativeEvent {
   
    pub fn emit_option_bought(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_bought"),
            attr("id", id.to_string()),
            //attr("price", price),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::BuyOption { id, }, "option_bought", attrs)
    }

    pub fn emit_option_price_updated(deps: Deps, id: u64, /*price: Vec<Coin>*/) -> StdResult<()> {
        let attrs = vec![
            attr("action", "price_updated"),
            attr("id", id.to_string()),
            //attr("id", price[]),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::PriceUpdated { id, /*price*/ }, "price_updated", attrs)
    }

    pub fn emit_execute_option(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "execute_option"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExecuted { id }, "execute_option", attrs)
    }

    pub fn emit_fractional_buy(deps: Deps, id: u64, amount: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "fractional_buy"),
            attr("id", id.to_string()),
            attr("amount", amount.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::FractionalBuy { id, amount }, "fractional_buy", attrs)
    }

    pub fn emit_partial_execution(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "partial_execution"),
            attr("id", id.to_string()),
            
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::PartialExecution { id }, "partial_execution", attrs)
    }

    pub fn emit_bid_placed(deps: Deps, bid_id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "bid_placed"),
            attr("bid_id", bid_id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::BidPlaced { bid_id }, "bid_placed", attrs)
    }

    pub fn emit_offer_placed(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "offer_placed"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OfferPlaced { id }, "offer_placed", attrs)
    }

    pub fn emit_bid_or_offer_accepted(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "bid_accepted"),
            attr("id", id.to_string()),
            //attr("option_id", option_id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::BidOrOfferAccepted { id }, "bid_or_offer_accepted", attrs)
    }

    pub fn emit_withdraw_collateral(deps: Deps, sender: Addr, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "withdraw_collateral"),
            attr("sender", sender.to_string()),
            attr("id", id.to_string()),
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::WithdrawCollateral { sender, id }, "withdraw_collateral", attrs)
    }

    pub fn emit_expiration_extended(deps: Deps, id: u64, new_expiration: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "expiration_extended"),
            attr("id", id.to_string()),
            attr("new_expiration", new_expiration.to_string()),
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::ExtendExpiration{ id, new_expiration }, "expiration_extended", attrs)
    }

        // Event for pausing the contract
    pub fn emit_contract_paused(deps: Deps) -> StdResult<()> {
        let attrs = vec![
            attr("action", "contract_paused"),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::ContractPaused {}, "contract_paused", attrs)
    }

    // Event for unpausing the contract
    pub fn emit_contract_unpaused(deps: Deps) -> StdResult<()> {
        let attrs = vec![
            attr("action", "contract_unpaused"),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::ContractUnpaused {}, "contract_unpaused", attrs)
    }

        // Event for adding an oracle
    pub fn emit_oracle_added(deps: Deps, sender: Addr, oracle: Addr) -> StdResult<()> {
        let attrs = vec![
            attr("action", "oracle_added"),
            attr("sender", sender.to_string()),
            attr("oracle", oracle.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OracleAdded { sender, oracle }, "oracle_added", attrs)
    }

    // Event for updating price with oracle
    pub fn emit_price_updated_with_oracle(deps: Deps, price: Vec<Coin>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "price_updated_with_oracle"),
            attr("price", format!("{:?}", price)),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::PriceUpdatedWithOracle { price }, "price_updated_with_oracle", attrs)
    }

    // Event for setting option exercise conditions
    pub fn emit_option_exercise_conditions_set(deps: Deps, id: u64, exercise_conditions: Vec<String>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_exercise_conditions_set"),
            attr("id", id.to_string()),
            attr("exercise_conditions", format!("{:?}", exercise_conditions)),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExerciseConditionsSet { id, exercise_conditions }, "option_exercise_conditions_set", attrs)
    }

    pub fn emit_option_expired_notification(
        deps: Deps,
        id: u64,
        notification_period: u64,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_expired_notification"),
            attr("id", id.to_string()),
            attr("notification_period", notification_period.to_string()),
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionExpiredNotification {
                id,
                notification_period,
            }, "expired_notification",
            attrs,
        )
    }

    pub fn emit_option_parameters_set(
        deps: Deps,
        id: u64,
        parameters: HashMap<String, String>,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_parameters_set"),
            attr("id", id.to_string()),
            attr("parameters", format!("{:?}", parameters)), // Format HashMap as a string for clarity
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionParametersSet { id, parameters }, "option_parameters_set",
            attrs,
        )
    }

    pub fn emit_liquidity_provided(deps: Deps, provider: String, amount: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "liquidity_provided"),
            attr("provider", provider.clone()),
            attr("amount", amount.clone()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::LiquidityProvided { provider, amount }, "liquidity_provided", attrs)
    }

    pub fn emit_liquidity_withdrawn(deps: Deps, provider: String, amount: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "liquidity_withdrawn"),
            attr("provider", provider.clone()),
            attr("amount", amount.clone()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::LiquidityWithdrawn { provider, amount }, "liquidity_withdrawn" , attrs)
    }
    
    pub fn emit_option_history(deps: Deps, id: u64, history: Vec<String>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_history"),
            attr("id", id.to_string()),
            attr("history", history.join(", ")), // Join history into a single string for clarity
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::OptionHistory { id, history }, "option_history", attrs)
    }
    
    pub fn emit_option_risk_metrics_calculated(deps: Deps, id: u64, risk_metrics: HashMap<String, Decimal>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_risk_metrics_calculated"),
            attr("id", id.to_string()),
            attr("risk_metrics", format!("{:?}", risk_metrics)), // Format HashMap as a string for clarity
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::OptionRiskMetricsCalculated { id, risk_metrics }, "option_risk_metrics_calculated", attrs)
    }
    

    pub fn emit_vote_on_governance_proposal(
        deps: Deps,
        proposal_id: u64,
        vote: bool,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "vote_on_governance_proposal"),
            attr("proposal_id", proposal_id.to_string()),
            attr("vote", vote.to_string()),
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::VoteOnGovernanceProposal { proposal_id, vote }, "vote_on_governance_proposal",
            attrs,
        )    
    }

    pub fn emit_option_used_as_collateral(
        deps: Deps,
        option_id: u64,
        amount: Vec<Coin>,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_used_as_collateral"),
            attr("option_id", option_id.to_string()),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionUsedAsCollateral { option_id, amount }, "option_used_as_collateral", 
            attrs,
        )
    }
    
    pub fn emit_option_wrapped_for_yield_farming(
        deps: Deps,
        option_id: u64,
        amount: Vec<Coin>,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_wrapped_for_yield_farming"),
            attr("option_id", option_id.to_string()),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionWrappedForYieldFarming { option_id, amount }, "option_wrapped_for_yield_farming",
            attrs,
        )
    }

    pub fn emit_amm_pool_created(deps: Deps, option_id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "amm_pool_created"),
            attr("option_id", option_id.to_string()),
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::AmmPoolCreated { option_id }, "amm_pool_created",
            attrs,
        )
    }
    
    pub fn emit_option_traded_on_amm(deps: Deps, pool_id: u64, amount: Vec<Coin>,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_traded_on_amm"),
            attr("pool_id", pool_id.to_string()),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::OptionTradedOnAmm { pool_id, amount }, "option_traded_on_amm", attrs,
        )
    }
    
    pub fn emit_market_data_feed_integrated(deps: Deps, option_id: u64, data_feed_url: String,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "market_data_feed_integrated"),
            attr("option_id", option_id.to_string()),
            attr("data_feed_url", data_feed_url.clone()),
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::MarketDataFeedIntegrated { option_id, data_feed_url }, "market_data_feed_integrated", attrs,
        )
    }

    
    pub fn emit_user_referred(deps: Deps, referred_user: Addr) -> StdResult<()> {
        let attrs = vec![
            attr("action", "user_referred"),
            attr("referred_user", referred_user.clone()),
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::UserReferred { referred_user }, "user_referred", attrs,
        )
    }
    
    pub fn emit_discount_criteria_set(deps: Deps, user: Addr, criteria: HashMap<String, String>,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "discount_criteria_set"),
            attr("user", user.clone()),
            attr("criteria", format!("{:?}", criteria)), // Format HashMap as a string for clarity
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::DiscountCriteriaSet { user, criteria }, "discount_criteria_set", attrs,
        )
    }

    pub fn emit_option_created(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_created"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionCreated { id }, "option_created",attrs)
    }

    pub fn emit_option_claimed(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_claimed"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionClaimed { id }, "option_claimed", attrs)
    }

    pub fn emit_option_transferred(deps: Deps, id: u64, new_owner: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_transferred"),
            attr("id", id.to_string()),
            attr("new_owner", new_owner.clone()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionTransferred { id, new_owner }, "option_transferred", attrs)
    }

    pub fn emit_option_expired(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_expired"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExpired { id }, "option_expired", attrs)
    }

    pub fn emit_option_added_to_market(
        deps: Deps,
        id: u64,
        //amount: String,
        //denom: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_added_to_market"),
            attr("id", id.to_string()),
            //attr("amount", amount.clone()),
            //attr("denom", denom.clone()),
        ];

        Self::log_event(
            deps.into(),
            ConstellationDerivativeEvent::OptionAddedToMarket { id, /*amount, denom*/ }, "option_added_to_market",
            attrs,
        )
    }

    pub fn emit_option_burned(
        deps: Deps,
        id: u64,
        sender: String,
        //creator: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_burned"),
            attr("id", id.to_string()),
            attr("sender", sender.clone()),
            //attr("creator", creator.clone()),
        ];

        Self::log_event(
            deps.into(),
            ConstellationDerivativeEvent::OptionBurned {
                id,
                sender,
              //  creator,
            }, "option_burned",
            attrs,
        )
    }

    pub fn emit_option_removed_from_market(
        deps: Deps,
        id: u64,
        //sender: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_removed_from_market"),
            attr("id", id.to_string()),
            //attr("sender", sender.clone()),
        ];

        Self::log_event(
            deps.into(),
            ConstellationDerivativeEvent::OptionRemovedFromMarket { id, /*sender*/ }, "option_removed_from_market",
            attrs,
        )
    }

   /* pub fn emit_bid_withdrawn(deps: Deps, bid_id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "bid_withdrawn"),
            attr("bid_id", bid_id.to_string()),
        ];
        Self::log_event(deps, ConstellationDerivativeEvent::BidWithdrawn { bid_id }, attrs)
    }

    pub fn emit_tokens_locked(
        deps: Deps, amount: Vec<Coin>, lock_duration: u64,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "tokens_locked"),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
            attr("lock_duration", lock_duration.to_string()),
        ];
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::TokensLocked { amount, lock_duration }, "tokens_locked" ,
            attrs,
        )
    }
*/

    fn log_event(_deps: Deps, event: ConstellationDerivativeEvent, event_type: &str, attrs: Vec<Attribute>) -> StdResult<()> {
        // Serialize the event data into a string
        //let event_data_str = serde_json::to_string(&event)?;

        let event_data_str = match serde_json::to_string(&event) {
            Ok(json) => json,
            Err(err) => {
              let msg = format!("{}", err);
              return Err(cosmwasm_std::StdError::generic_err(msg));
            }
          };    
        // Add the serialized data as an attribute
        let mut custom_attrs = attrs;
        custom_attrs.push(Attribute {
            key: "data".to_string(),
            value: event_data_str,
        });
        // Create a new event with the given type and attributes
        let custom_event = Event::new(event_type).add_attributes(custom_attrs);
        let custom_event_name = format!("ConstellationDerivativeEvent_{}", event_type);
        // Create a response and add the custom event to it
        let response: Response<()> = Response::new().add_event(custom_event);
        // Add default "wasm" event with additional attributes if needed
        response.add_attribute("action", custom_event_name);
        // Return the response
        //Ok(response)
        Ok(())
    }
}

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

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

fn query_options(deps: Deps)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> =OPTION_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let resp =options?;
    Ok(resp)
}

fn query_options_page(deps: Deps,key: u64,amount:u64)->StdResult<OptionsResponse>{
    let limit = amount as usize;
    let options:StdResult<Vec<_>> =OPTION_LIST.range(deps.storage, Some(Bound::exclusive(key)), None, Order::Ascending).take(limit).collect();
    let resp =options?;
    Ok(resp)
}

fn query_option_by_id(deps: Deps,id: u64)->StdResult<GetOptionByIdResponse>{
    let option = OPTION_LIST.load(deps.storage, id);
    Ok(option?)
}

fn query_market_options(deps: Deps)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> =MARKET_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let resp =options?;
    Ok(resp)

}

fn query_market_options_page(deps: Deps,key: u64,amount:u64)->StdResult<OptionsResponse>{
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

//Note: Tests for functions with placeholder logic may fail at this point
//Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, Timestamp, CosmosMsg};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        let info = mock_info("creator", &coins(0, ""));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!("creator", res.creator.as_str());
        assert_eq!(0, res.total_options_num);

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-2", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!(2, res.total_options_num);
    }

    #[test]
    fn create_and_query_options() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(0, "")),
            msg,
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-2", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        let res = query_options(deps.as_ref()).unwrap();
        let aim_data = vec![
            (
                0,
                Data {
                    creator: Addr::unchecked("creator-1".to_string()),
                    owner: Addr::unchecked("creator-1".to_string()),
                    collateral: coins(100, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
            (
                1,
                Data {
                    creator: Addr::unchecked("creator-2".to_string()),
                    owner: Addr::unchecked("creator-2".to_string()),
                    collateral: coins(100, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
        ];
        assert_eq!(aim_data, res);

        let wrong_data = vec![
            (
                0,
                Data {
                    creator: Addr::unchecked("creator-1".to_string()),
                    owner: Addr::unchecked("creator-1".to_string()),
                    collateral: coins(100, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
            (
                1,
                Data {
                    creator: Addr::unchecked("creator-1".to_string()),
                    owner: Addr::unchecked("creator-1".to_string()),
                    collateral: coins(90, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
        ];
        assert_ne!(wrong_data, res);
    }

    #[test]
    fn transfer() -> Result<(), String> {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        let res = query_options(deps.as_ref()).unwrap();
        let aim_data = vec![(
            0,
            Data {
                creator: Addr::unchecked("creator-1".to_string()),
                owner: Addr::unchecked("creator-1".to_string()),
                collateral: coins(100, "ETH"),
                counter_offer: coins(100, "BTC"),
                onsale: false,
                expires: Timestamp::from_seconds(11692624898),
                price: Vec::new(),
                highest_bidder: None,
                best_offer: None,
                bid_history: None,
                status: OptionStatus::Active,
                parameters: None,
                exercise_conditions: None,
                history: Vec::new(),
                risk_metrics: None,
                pool_share: Uint128::new(0),
            },
        )];
        assert_eq!(aim_data, res);
    
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Transfer {
                id: 0,
                to: "creator-2".to_string(),
            },
        )
        .unwrap();
    
        let res = query_options(deps.as_ref()).unwrap();
        let aim_data = vec![(
            0,
            Data {
                creator: Addr::unchecked("creator-1".to_string()),
                owner: Addr::unchecked("creator-2".to_string()),
                collateral: coins(100, "ETH"),
                counter_offer: coins(100, "BTC"),
                onsale: false,
                expires: Timestamp::from_seconds(11692624898),
                price: Vec::new(),
                highest_bidder: None,
                best_offer: None,
                bid_history: None,
                status: OptionStatus::Active,
                parameters: None,
                exercise_conditions: None,
                history: Vec::new(),
                risk_metrics: None,
                pool_share: Uint128::new(0),
            },
        )];
        assert_eq!(aim_data, res);
    
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Transfer {
                id: 0,
                to: "creator-2".to_string(),
            },
        );
    
        match res {
            Ok(_) => Err("validate the auth wrong".to_string()),
            Err(error) => {
                if let ContractError::Unauthorized {} = error {
                    Ok(())
                } else {
                    Err("wrong error type".to_string())
                }
            }
        }
    }
    
    #[test]
    fn market_action() -> Result<(), String> {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("bob", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        // test add to market
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::AddToMarket {
                id: 0,
                amount: 100,
                denom: "usdc".to_string(),
            },
        )
        .unwrap();
    
        let res = query_market_options(deps.as_ref()).unwrap();
        let res_fr_options = query_option_by_id(deps.as_ref(), 0).unwrap();
        let aim_data = vec![(
            0,
            Data {
                creator: Addr::unchecked("alice".to_string()),
                owner: Addr::unchecked("alice".to_string()),
                collateral: coins(100, "ETH"),
                counter_offer: coins(100, "BTC"),
                onsale: true,
                price: coins(100, "usdc"),
                expires: Timestamp::from_seconds(11692624898),
                highest_bidder: None,
                best_offer: None,
                bid_history: None,
                status: OptionStatus::Active,
                parameters: None,
                exercise_conditions: None,
                history: Vec::new(),
                risk_metrics: None,
                pool_share: Uint128::new(0),
            },
        )];
        assert_eq!(aim_data, res);
        assert_eq!(aim_data[0].1, res_fr_options);
    
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("bob", &coins(100, "ETH")),
            ExecuteMsg::AddToMarket {
                id: 0,
                amount: 100,
                denom: "usdc".to_string(),
            },
        );
    
        let _ = match res {
            Ok(_) => Err::<(), String>("validate the auth wrong".to_string()),
            Err(error) => {
                if let ContractError::Unauthorized {} = error {
                    return Ok(());
                } else {
                    return Err("wrong error type".to_string());
                }
            }
        };
    
        // test remove
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::RemoveFromMarket { id: 0 },
        )
        .unwrap();
    
        let res = query_market_options(deps.as_ref()).unwrap();
        let empty_vec: Vec<(u64, Data)> = Vec::new();
        assert_eq!(res, empty_vec);
    
        // test buy and update price
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::AddToMarket {
                id: 0,
                amount: 100,
                denom: "usdc".to_string(),
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            ExecuteMsg::UpdatePrice { id: 0, price: coins(120, "usdc") },
        )
        .unwrap();
    
        let res = query_option_by_id(deps.as_ref(), 0).unwrap();
        let aim_data = Data {
            creator: Addr::unchecked("alice".to_string()),
            owner: Addr::unchecked("alice".to_string()),
            collateral: coins(100, "ETH"),
            counter_offer: coins(100, "BTC"),
            onsale: true,
            price: coins(120, "usdc"),
            expires: Timestamp::from_seconds(11692624898),
            highest_bidder: None,
            best_offer: None,
            bid_history: None,
            status: OptionStatus::Active,
            parameters: None,
            exercise_conditions: None,
            history: Vec::new(),
            risk_metrics: None,
            pool_share: Uint128::new(0),
        };
        assert_eq!(res, aim_data);
    
        execute(deps.as_mut(), mock_env(), mock_info("bob", &coins(120, "usdc")), ExecuteMsg::Buy { id: 0 }).unwrap();
        let res = query_option_by_id(deps.as_ref(), 0).unwrap();
        let aim_data = Data {
            creator: Addr::unchecked("alice".to_string()),
            owner: Addr::unchecked("bob".to_string()),
            collateral: coins(100, "ETH"),
            counter_offer: coins(100, "BTC"),
            onsale: false,
            expires: Timestamp::from_seconds(11692624898),
            price: Vec::new(),
            highest_bidder: None,
            best_offer: None,
            bid_history: None,
            status: OptionStatus::Active,
            parameters: None,
            exercise_conditions: None,
            history: Vec::new(),
            risk_metrics: None,
            pool_share: Uint128::new(0),
        };
        assert_eq!(res, aim_data);
    
        Ok(())
    }
    
    
 #[test]
    fn burn(){
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: 11692624898 }).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("bob",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: 11692624898 }).unwrap();

        // expired returns funds
        let mut env = mock_env();
        env.block.height = 200_000;
        let res = execute_burn(deps.as_mut(), mock_info("alice", &coins(0, "")),0).unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "ETH"),
            })
        );

        // check deleted
        let _ = query_option_by_id(deps.as_ref(),0).unwrap_err();
    }

    #[test]
    fn execute_option(){
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: 11692624898 }).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), ExecuteMsg::Transfer { id: 0, to: "bob".to_string() }).unwrap();
        let res = execute(deps.as_mut(), mock_env(), mock_info("bob", &coins(100, "BTC")), ExecuteMsg::Execute { id: 0 }).unwrap();
        assert_eq!(res.messages.len(), 2);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "BTC"),
            })
        );
        assert_eq!(
            res.messages[1].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "bob".into(),
                amount: coins(100, "ETH"),
            })
        );
    }
    #[test]
    fn cliam(){
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+2 }).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(0,"")), ExecuteMsg::Claim { id: 0 }).unwrap_err();
        let mut now_env = mock_env();
        now_env.block.time = Timestamp::from_seconds(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+5);
        let res = execute(deps.as_mut(), now_env, mock_info("alice",&coins(0,"")), ExecuteMsg::Claim { id: 0 }).unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "ETH"),
            })
        );
    }
   
    #[test]
    fn place_bid_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Create an option
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Place a bid
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(200_000); // set a time in the future to pass the time validation
        let res = execute_place_bid(
            deps.as_mut(),
            env,
            mock_info("bob", &coins(100, "BTC")),
            0,
            coins(100, "BTC"), // assuming the bid amount is the same as the counter_offer
        )
        .unwrap();

        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "place_bid");
    }
    
    
    

    #[test]
    fn place_offer_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        // Place an offer
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(200_000); // set a time in the future to pass the time validation
        let res = execute_place_offer(
            deps.as_mut(),
            env,
            mock_info("bob", &coins(100, "BTC")),
            0,
            coins(100, "BTC"), // assuming the offer amount is the same as the counter_offer
        )
        .unwrap();
    
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "place_offer");
    }
    
    #[test]
    fn accept_bid_or_offer_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        // Accept bid or offer
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(200_000); // set a time in the future to pass the time validation
        let res = execute_accept_bid_or_offer(deps.as_mut(), env, 0);
    
        match res {
            Ok(_) => {
                assert!(false, "Expected an error, but received Ok");
            }
            Err(error) => {
                if let ContractError::NoBidOrOffer = error {
                    // This is the expected error
                } else {
                    panic!("Unexpected error type: {:?}", error);
                }
            }
        }
    }
    


    #[test]
    fn execute_partial_test() {
        // Initialize mock dependencies
        let mut deps = mock_dependencies();
    
        // Instantiate the contract
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        // Execute partial
        let res = execute_partial(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            0.5,
        )
        .unwrap();
    
        // Validate the response
        assert_eq!(res.messages.len(), 2);
        
        // Validate the first message for partial collateral transfer
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(50, "ETH"),
            })
        );
    
        // Validate the second message for partial counter offer transfer
        assert_eq!(
            res.messages[1].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(50, "BTC"),
            })
        );
    }
    

    #[test]
    fn execute_buy_fraction_test() {
        // Initialize mock dependencies
        let mut deps = mock_dependencies();
    
        // Instantiate the contract
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option in the market
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        // Execute buy fraction
        let res = execute_buy_fraction(
            deps.as_mut(),
            mock_env(),
            mock_info("bob", &coins(100, "ETH")),
            0,
            0.5,
        );
        
        // Print or log the result for debugging
        println!("{:?}", res);

        // Validate the result
        assert!(res.is_ok());
    
        // Add additional assertions or checks if needed
    }
    
    #[test]
    fn extend_expiration_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        let res = execute_extend_expiration(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), 0, 11792624898);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn pause_and_unpause_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();

        let res_pause = execute_pause(deps.as_mut(), mock_info("admin", &coins(0, "")));
        assert!(res_pause.is_ok());

        let res_unpause = execute_unpause(deps.as_mut(), mock_info("admin", &coins(0, "")));
        assert!(res_unpause.is_ok());

        // Add assertions or additional checks if needed
    }


    #[test]
    fn add_oracle_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();
        let res = execute_add_oracle(deps.as_mut(), mock_info("admin", &coins(0, "")), Addr::unchecked("oracle"));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn update_price_oracle_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Update price oracle
        let res = execute_update_price_oracle(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), 0, coins(150, "ETH"));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    
    #[test]
    fn execute_set_option_parameters_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Set option parameters
        let mut parameters = HashMap::new();
        parameters.insert("param1".to_string(), "value1".to_string());
        parameters.insert("param2".to_string(), "value2".to_string());

        let res = execute_set_option_parameters(
            deps.as_mut(),
            mock_info("creator", &coins(0, "")),
            0,
            parameters.clone(),
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_notify_option_expiry_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Notify option expiry
        let res = execute_notify_option_expiry(deps.as_mut(), mock_env(), 0, 10);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_get_option_history_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Get option history
        let res = execute_get_option_history(deps.as_mut(), 0);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
    
    #[test]
    fn execute_calculate_option_risk_metrics_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Calculate option risk metrics
        let res = execute_calculate_option_risk_metrics(deps.as_mut(), 0);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_provide_liquidity_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Provide liquidity
        let res = execute_provide_liquidity(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(100, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_withdraw_liquidity_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Withdraw liquidity
        let res = execute_withdraw_liquidity(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
    
    
    #[test]
    fn execute_vote_on_governance_proposal_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Vote on governance proposal
        let res = execute_vote_on_governance_proposal(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), 0, true);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_use_option_as_collateral_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Use option as collateral
        let res = execute_use_option_as_collateral(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_wrap_option_for_yield_farming_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Wrap option for yield farming
        let res = execute_wrap_option_for_yield_farming(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
        }
    
    
    #[test]
    fn execute_create_amm_pool_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Create AMM pool
        let res = execute_create_amm_pool(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), 0);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_trade_on_amm_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Trade on AMM
        let res = execute_trade_on_amm(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_integrate_market_data_feed_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        // Integrate market data feed
        let res = execute_integrate_market_data_feed(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            "https://example.com/data-feed".to_string(),
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
        
    #[test]
    fn execute_refer_user_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Refer user
        let res = execute_refer_user(deps.as_mut(), mock_info("alice", &coins(0, "")), Addr::unchecked("bob"));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_set_discount_criteria_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Set discount criteria
        let mut criteria = HashMap::new();
        criteria.insert("param1".to_string(), "value1".to_string());
        criteria.insert("param2".to_string(), "value2".to_string());

        let res = execute_set_discount_criteria(deps.as_mut(), mock_info("alice", &coins(0, "")), criteria);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
        
        
        
    /*

    //Test for creating tokens eg ERC 20 tokens and locking tokens - to be implemented
    #[test]
    fn execute_create_token_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();

        // Create token
        let res = execute_create_token(deps.as_mut(), mock_info("admin", &coins(0, "")), Coin {
            denom: "usdc".to_string(),
            amount: "100".to_string(),
        }, Timestamp::from_seconds(11692624898));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_lock_tokens_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Lock tokens
        let res = execute_lock_tokens(deps.as_mut(), mock_env(), mock_info("alice", &coins(100, "TOKEN")), vec![coin(100, "TOKEN")], 10);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

*/
}