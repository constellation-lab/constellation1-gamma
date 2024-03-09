
#[allow(unused_imports)]
use cosmwasm_std::{QueryRequest,WasmQuery,WasmMsg,SubMsg,
    Addr,entry_point, to_json_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin,Timestamp, Uint128};
use constellation::msg::{QueryMsg as OPQueryMsg, ExecuteMsg as OPExecutemsg,GetOptionByIdResponse};
use crate::state::{ContractParams, CONTRACT_PARAMS,LISTITEM_LIST,BID_LIST, ListItem, Bid};
use crate::msgs::{ ExecuteMsg, InstantiateMsg}; 
use crate::error::ContractError;

pub const DENOM:&str = "unibi";

#[cfg_attr(not(feature = "library"), entry_point)]
#[allow(dead_code)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let option_addr = deps.api.addr_validate(&msg.option_address)?;

    let state = ContractParams {
        option_address: option_addr,
        onwer_address: info.sender,
    };
    CONTRACT_PARAMS.save(deps.storage, &state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
#[allow(dead_code)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::List { id, price, expires } => execute_list(deps, env, info, id, price, expires),
        ExecuteMsg::RemoveList { id }=> execute_remove_list(deps, env, info, id),
        ExecuteMsg::SetBid { id, expires }=> execute_set_bid(deps, env, info, id,expires),
        ExecuteMsg::RemoveBid { id } =>execute_remove_bid(deps, info, id),
        ExecuteMsg::AcceptBid { id, bidder }=> execute_accept_bid(deps, env, info, id,bidder),
        ExecuteMsg::Buy { id } => execute_buy(deps, env, info, id),
        ExecuteMsg::UpdatePrice { id, price } => execute_update_price(deps, env, info, id,price),
    }
}

pub fn execute_list(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
    price: Uint128,
    expires: u64,
) -> Result<Response, ContractError> {
    let params = CONTRACT_PARAMS.load(deps.storage)?;
    let option = get_option_info(deps.as_ref(), &info.clone(), &params.option_address.clone(), id)?;
    let is_approve = get_is_approve(deps.as_ref(), &params.option_address.clone(), &env.contract.address,&option.owner.clone())?;
    if option.isBurned || option.expires < env.block.time{
        return Err(ContractError::OptionIsburnedOrexpired{});
    }
    if !is_approve || option.owner != info.sender{
        return Err(ContractError::UnauthorizedOperator {});
    }
    let expires = Timestamp::from_seconds(expires);

    if expires < env.block.time {
        return Err(ContractError::ListItemIsExpired{});
    }
    let item = ListItem{
        seller: option.owner.clone(),
        price: price,
        expires_at: expires,
    };
    LISTITEM_LIST.save(deps.storage, id, &item).unwrap();
    let res = Response::new();
    return Ok(res);
}

pub fn execute_accept_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
    bidder: String,
)->Result<Response, ContractError>{
    let params = CONTRACT_PARAMS.load(deps.storage)?;
    let option = get_option_info(deps.as_ref(), &info.clone(), &params.option_address.clone(), id)?;
    let is_approve = get_is_approve(deps.as_ref(), &params.option_address.clone(), &env.contract.address,&option.owner.clone())?;
    if option.isBurned || option.expires < env.block.time{
        return Err(ContractError::OptionIsburnedOrexpired{});
    }
    if !is_approve || option.owner != info.sender{
        return Err(ContractError::UnauthorizedOperator {});
    }
    let bider_addr = deps.api.addr_validate(&bidder)?;
    let bid = BID_LIST.load(deps.storage, (id,bider_addr))?;
    let mut res = Response::new();
    let send_to_owner = Coin{denom:DENOM.to_string(),amount: bid.price.clone().multiply_ratio(Uint128::new(98), Uint128::new(100))};
    let fees = Coin{denom:DENOM.to_string(),amount: bid.price.clone().multiply_ratio(Uint128::new(2), Uint128::new(100))};

    res = res.add_message(BankMsg::Send { to_address: option.owner.to_string(), amount: vec![send_to_owner]});
    res = res.add_message(BankMsg::Send { to_address: params.onwer_address.to_string(), amount: vec![fees]});
    let option_transfer_msg = OPExecutemsg::Transfer { id: id, to: bidder};
    let exec_option_transfer = WasmMsg::Execute {
        contract_addr: params.option_address.into_string(),
        msg: to_json_binary(&option_transfer_msg)?,
        funds: vec![],
    };
    res.messages.push(SubMsg::new(exec_option_transfer));
    Ok(res)
}

pub fn execute_buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
)->Result<Response, ContractError>{
    let params = CONTRACT_PARAMS.load(deps.storage)?;
    let option = get_option_info(deps.as_ref(), &info.clone(), &params.option_address.clone(), id)?;
    if option.isBurned || option.expires < env.block.time{
        return Err(ContractError::OptionIsburnedOrexpired{});
    }
    let item = LISTITEM_LIST.load(deps.storage, id)?;
    if info.funds[0].denom != DENOM || info.funds[0].amount != item.price{
        return Err(ContractError::InvalidPrice {});
    }

    let mut res = Response::new();
    let send_to_owner = Coin{denom:DENOM.to_string(),amount: item.price.clone().multiply_ratio(Uint128::new(98), Uint128::new(100))};
    let fees = Coin{denom:DENOM.to_string(),amount: item.price.clone().multiply_ratio(Uint128::new(2), Uint128::new(100))};

    res = res.add_message(BankMsg::Send { to_address: option.owner.to_string(), amount: vec![send_to_owner]});
    res = res.add_message(BankMsg::Send { to_address: params.onwer_address.to_string(), amount: vec![fees]});
    let option_transfer_msg = OPExecutemsg::Transfer { id: id, to: info.sender.to_string()};
    let exec_option_transfer = WasmMsg::Execute {
        contract_addr: params.option_address.into_string(),
        msg: to_json_binary(&option_transfer_msg)?,
        funds: vec![],
    };
    LISTITEM_LIST.remove(deps.storage, id);
    res.messages.push(SubMsg::new(exec_option_transfer));
    Ok(res)
}


pub fn execute_remove_list(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
)->Result<Response, ContractError>{
    let params = CONTRACT_PARAMS.load(deps.storage)?;
    let option = get_option_info(deps.as_ref(), &info.clone(), &params.option_address.clone(), id)?;
    let is_approve = get_is_approve(deps.as_ref(), &params.option_address.clone(), &env.contract.address,&option.owner.clone())?;
    if !is_approve || option.owner != info.sender{
        return Err(ContractError::UnauthorizedOperator {});
    }
    LISTITEM_LIST.remove(deps.storage, id);
    let res = Response::new();
    return Ok(res);
}

pub fn execute_update_price(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
    price: Uint128,
)->Result<Response, ContractError>{
    let params = CONTRACT_PARAMS.load(deps.storage)?;
    let option = get_option_info(deps.as_ref(), &info.clone(), &params.option_address.clone(), id)?;
    let is_approve = get_is_approve(deps.as_ref(),  &params.option_address.clone(), &env.contract.address,&option.owner.clone())?;
    if !is_approve || option.owner != info.sender{
        return Err(ContractError::UnauthorizedOperator {});
    }
    let mut item = LISTITEM_LIST.load(deps.storage, id)?;
    item.price = price;
    LISTITEM_LIST.save(deps.storage, id, &item).unwrap();
    let res = Response::new();
    return Ok(res);
}


pub fn execute_set_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
    expires: u64,
)->Result<Response, ContractError>{
    let params = CONTRACT_PARAMS.load(deps.storage)?;
    let option = get_option_info(deps.as_ref(), &info.clone(), &params.option_address.clone(), id)?;
    let expires = Timestamp::from_seconds(expires);
    if option.isBurned || option.expires < env.block.time{
        return Err(ContractError::OptionIsburnedOrexpired{});
    }
    if info.funds[0].denom != DENOM{
        return Err(ContractError::InvalidToken {});
    }
    let bid = Bid{
        price: info.funds[0].amount,
        expires_at: expires,
    };
    BID_LIST.save(deps.storage, (id,info.sender), &bid).unwrap();
    let res = Response::new();
    return Ok(res);
}

pub fn execute_remove_bid(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
)->Result<Response, ContractError>{
    BID_LIST.remove(deps.storage, (id,info.sender));
    let res = Response::new();
    return Ok(res);
}


///Checks the sender is approve to control the options
fn get_option_info(
    deps: Deps,
    info: &MessageInfo,
    option_contract: &Addr,
    token_id: u64,
) -> Result<GetOptionByIdResponse, ContractError> {
    let query_msg: OPQueryMsg =OPQueryMsg::GetOptionByid {id: token_id};
    let query_response: GetOptionByIdResponse =
            deps.querier.query_wasm_smart(option_contract.clone(), &query_msg)?;
    return Ok(query_response);
}

///Checks the sender is approve to control the options
fn get_is_approve(
    deps: Deps,
    option_contract: &Addr,
    spender: &Addr,
    owner: &Addr,
) -> Result<bool, ContractError> {
    let query_msg: OPQueryMsg =OPQueryMsg::GetIsApprove{spender: spender.clone().into_string(),owner: owner.clone().into_string()};
    let query_response: bool =
            deps.querier.query_wasm_smart(option_contract.clone(), &query_msg)?;
    return Ok(query_response);
}
