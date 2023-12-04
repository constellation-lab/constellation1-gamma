use crate::events::ConstellationDerivativeEvent;
#[allow(unused_imports)]
use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin,Timestamp, Uint128};
use crate::state::{State, CONFIG, Data,OPTION_LIST,CREATOR_LIST,OWNER_LIST,MARKET_LIST, OptionStatus};
use crate::msg::{ ExecuteMsg, InstantiateMsg}; 
use crate::error::ContractError;
use crate::operations::*;
//use maplit::hashmap;  //use serde_json::to_string;


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




