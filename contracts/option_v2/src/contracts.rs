use crate::events::ConstellationDerivativeEvent;
#[allow(unused_imports)]
use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin,Timestamp, Uint128};
use crate::state::{State, CONFIG, Data,OPTION_LIST,APPROVE_LIST};
use crate::msg::{ ExecuteMsg, InstantiateMsg}; 
use crate::error::ContractError;
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
        owner: info.sender.clone(),
        total_options_amount: 0,
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
        ExecuteMsg::Create{counter_offer,time_stamp} => execute_create(deps, env, info, counter_offer[0],time_stamp),
        ExecuteMsg::Burn { id }=> execute_burn(deps, info,id),
        ExecuteMsg::ClaimCollateral { id } => execute_claim(deps,env,id),
        ExecuteMsg::Transfer {id, to } => execute_transfer(deps, env, info,id, to),
        ExecuteMsg::Execute { id}=>execute_execute(deps, env, info, id),
        ExecuteMsg::Split { id, percentage } => execute_split(deps,env,info,id,percentage),
        ExecuteMsg::Approve { spender } => execute_approve(deps,env,info,spender),
        ExecuteMsg::DisApprove { spender } => execute_disapprove(deps,env,info,spender),
    }
}

pub fn execute_approve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
)-> Result<Response,ContractError>{
    let spender = deps.api.addr_validate(&spender)?;
    APPROVE_LIST.save(deps.storage, (info.sender,spender), &true);
    let res: Response =
        Response::new().add_attributes([("action", "approve"),("owner", &info.sender.to_string()), ("spender", spender.as_str())]);
    Ok(res)
}

pub fn execute_disapprove(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
)-> Result<Response,ContractError>{
    let spender = deps.api.addr_validate(&spender)?;
    APPROVE_LIST.save(deps.storage, (info.sender,spender), &false);
    let res: Response =
        Response::new().add_attributes([("action", "approve"),("owner", &info.sender.to_string()), ("spender", spender.as_str())]);
    Ok(res)
}

pub fn execute_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id:u64,
)-> Result<Response,ContractError>{
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    let state: State = CONFIG.load(deps.storage)?;
    //validate time and owner
    let is_apporve = _validatre_approve(deps, env, info, id);
    if !is_apporve {
        return Err(ContractError::Unauthorized {});
    }
    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    if info.funds[0] != option.counter_offer {
        return Err(ContractError::CounterOfferMismatch {
            offer: info.funds[0],
            counter_offer: option.counter_offer,
        });
    }
    let mut res: Response = Response::new();
    let send_to_owner = Coin{denom:option.collateral.denom,amount: option.collateral.amount.multiply_ratio(Uint128::new(98), Uint128::new(100))};
    let fees = Coin{denom:option.collateral.denom,amount: option.collateral.amount.multiply_ratio(Uint128::new(2), Uint128::new(100))};
    res = res.add_message(BankMsg::Send {
        to_address: option.creator.to_string(),
        amount: vec![option.counter_offer],
    });
    res = res.add_message(BankMsg::Send {
        to_address: option.owner.to_string(),
        amount: vec![send_to_owner],
    });
    res = res.add_message(BankMsg::Send { to_address:state.owner.to_string() , amount: vec![fees] });
    res = res.add_message(BankMsg::Send {
        to_address: option.owner.to_string(),
        amount: vec![option.collateral],
    });

    // Emit the Option executed event
    ConstellationDerivativeEvent::emit_execute_option(deps.as_ref(), id)?;
    option.isBurned = true;
    OPTION_LIST.save(deps.storage, id,&option);
    res = res.add_attribute("action", "execute");
    Ok(res)
}

pub fn execute_split(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id:u64,
    percentage:u64,
)-> Result<Response,ContractError>{
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    let mut state =  CONFIG.load(deps.storage)?;
    let is_approve = _validatre_approve(deps, env, info, id);
    if percentage>=100{
        return Err(ContractError::Over100{});
    }
    //validate time and owner
    if !is_approve {
        return Err(ContractError::Unauthorized {});
    }
    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }
    let key = state.total_options_amount;
    let new_collateral = Coin{denom:option.counter_offer.denom,amount: option.counter_offer.amount.multiply_ratio(Uint128::new(percentage as u128), Uint128::new(100))};
    let old_collateral = Coin{denom:option.counter_offer.denom,amount: option.counter_offer.amount-new_collateral.amount};

    let new_counter_toffer = Coin{denom:option.counter_offer.denom,amount: option.counter_offer.amount.multiply_ratio(Uint128::new(percentage as u128), Uint128::new(100))};
    let old_counter_offer = Coin{denom:option.counter_offer.denom,amount: option.counter_offer.amount-new_counter_toffer.amount};
    let new_data:Data = Data { 
        creator: option.creator.clone(), 
        owner: option.owner.clone(), 
        collateral: new_collateral, 
        counter_offer: new_counter_toffer, 
        expires:option.expires.clone(),
        isBurned: option.isBurned,
    };
    option.collateral = old_collateral;
    option.counter_offer = old_counter_offer;
    state.total_options_amount = key+1;
    OPTION_LIST.save(deps.storage, id, &option);
    OPTION_LIST.save(deps.storage, key, &new_data);
    CONFIG.save(deps.storage, &state);
    OPTION_LIST.remove(deps.storage, id);
    let res: Response =
    Response::new().add_attributes([("action", "create option"), ("id", &key.to_string())]); 
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
    let is_approve = _validatre_approve(deps, env, info, id);
    if !is_approve{
        return Err(ContractError::Unauthorized {});
    }
    if env.block.time >= option.expires {
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    //update option
    option.owner = deps.api.addr_validate(&recipient)?;

     // Emit the Option transferred event
     ConstellationDerivativeEvent::emit_option_transferred(deps.as_ref(), id, info.sender.to_string())?;

    OPTION_LIST.save(deps.storage,id , &option)?;
    // set new owner on state
    
    let res: Response =
        Response::new().add_attributes([("action", "transfer"), ("owner", recipient.as_str())]);
    Ok(res)
}


//claim expired option, remove the option and payback the collateral to the creator
pub fn execute_claim(
    deps:DepsMut,
    env: Env,
    id: u64,
)->Result<Response,ContractError>{
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    if option.isBurned{
        return Err(ContractError::OptionIsBurned {});
    }
    if option.expires > env.block.time{
        return Err(ContractError::OptionNotExpired { expires: option.expires });
    }
    if option.isBurned{
        return Err(ContractError::OptionNotExpired { expires: option.expires });
    }
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send { to_address: option.creator.to_string(), amount: vec![option.collateral]}).add_attribute("action", "claim");
    option.isBurned = true;
    // Emit the event
    ConstellationDerivativeEvent::emit_option_claimed(deps.as_ref(), id)?;

    OPTION_LIST.save(deps.storage, id, &option);
    res = res.add_attribute("action", "claim");
    Ok(res)
}


pub fn  execute_create(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    counter_offer: Coin,
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
        collateral: info.funds[0], 
        counter_offer: counter_offer, 
        expires:expires,
        isBurned: false,
    };
    let mut state = CONFIG.load(deps.storage)?;

     //save the key id to the own and creator's list: key = id = option_id
     let key: u64 = state.total_options_amount;
  
    //OPTION_LIST.save(deps.storage, key, &new_data);
    OPTION_LIST.save(deps.storage,state.total_options_amount , &new_data)?;

    //save the total_option+1
    state.total_options_amount=key + 1;
    CONFIG.save(deps.storage, &state)?;

    // Emit the event
    ConstellationDerivativeEvent::emit_option_created(deps.as_ref(), key)?;

    let res: Response =
        Response::new().add_attributes([("action", "create option"), ("id", &key.to_string())]);
    Ok(res)
}

//burn the option and send the collateral  to the creator
pub fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
)->Result<Response, ContractError>{
    //load the option data
    let mut option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return Err(ContractError::Std(error)),
    };
    if option.isBurned{
        return Err(ContractError::OptionIsBurned {});
    }

    //validate the sender is the owner
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send {
        to_address: option.creator.to_string(),
        amount: vec![option.collateral],
    });

    ConstellationDerivativeEvent::emit_option_burned(deps.as_ref(), id, info.sender.to_string() )?;
    option.isBurned = true;
    OPTION_LIST.save(deps.storage, id, &option);
    res = res.add_attribute("action", "burn");
    Ok(res)
}

// validate message sender is appove to control the option, if approve return ture, disapprove return false
fn _validatre_approve(deps: DepsMut,env: Env,info: MessageInfo,id:u64)->bool{
    let option = match OPTION_LIST.load(deps.storage,id ){
        Ok(option)=> option,
        Err(error) => return false,
    };
    if option.isBurned{
        return false;
    }
    if info.sender == option.owner {
        return true;
    }
    let is_approve = match  APPROVE_LIST.load(deps.storage, (option.owner, info.sender)){
        Ok(isApprove)=>isApprove,
        Err(error)=> return false,
    };
    if is_approve{
        return  true;
    }
    false
}


