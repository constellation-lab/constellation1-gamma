use std::env;
use std::ops::Add;
use std::sync::mpsc::SendError;
use cw_storage_plus::Bound;

#[allow(unused_imports)]
use cosmwasm_std::{QueryRequest,WasmQuery,WasmMsg,SubMsg,
    Addr,entry_point, to_json_binary,Order, BankMsg, Deps, DepsMut, Env, MessageInfo, Response,Binary ,StdResult, Coin,Timestamp, Uint128};
use constellation::msg::{ExecuteMsg as OPExecutemsg, GetOptionByIdResponse, OptionsResponse, QueryMsg as OPQueryMsg};
use crate::state::{ContractParams, CONTRACT_PARAMS,LISTITEM_LIST,BID_LIST, ListItem, Bid};
use crate::msgs::{ ContractParamsResponse, ListItemData,GetListItemByIdResponse, ListItemsResponse,BidListRespose, QueryMsg}; 
use crate::error::ContractError;

//Queries
#[entry_point]
#[allow(dead_code)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractParams {} => to_json_binary(&query_config(deps)?),
        QueryMsg::ListItems {}=> to_json_binary(&query_list_item(deps,_env)?),
        QueryMsg::ListItemsPage { key, amount }=>to_json_binary(&query_list_item_page(deps,_env, key, amount)?),
        QueryMsg::GetListItemsByid { id }=>to_json_binary(&query_item_by_id(deps,_env,id)?),
        QueryMsg::OwnerListItems { addr }=>to_json_binary(&query_owner_list_items(deps, addr)?),
        QueryMsg::OwnerUnListItems { addr } =>to_json_binary(&query_owner_unlist_items(deps, addr)?),
        QueryMsg::BidList { id }=>to_json_binary(&query_bid_List(deps, id)?),
    }
}

pub fn query_bid_List(deps: Deps, id: u64) -> StdResult<BidListRespose>{
    let bids:StdResult<Vec<_>> = BID_LIST.prefix(id).range(deps.storage, None, None, Order::Ascending).collect();
    let resp =bids?;
    Ok(resp)
}

pub fn query_config(deps: Deps) -> StdResult<ContractParamsResponse> {
    let state = CONTRACT_PARAMS.load(deps.storage)?;
    Ok(state)
}

pub fn query_item_by_id(deps: Deps, _env: Env, id: u64)->StdResult<GetListItemByIdResponse>{
    let state = CONTRACT_PARAMS.load(deps.storage)?;
    let contrac_addr = state.option_address;
    let item = LISTITEM_LIST.load(deps.storage, id)?;
    let option = get_option_info(deps, &contrac_addr, id).unwrap();
    if option.isBurned ||_env.block.time > option.expires || option.owner != item.seller {
        return Err(cosmwasm_std::StdError::NotFound { kind: "not found the options or it's expired!!!".to_string() });
    }
    let item_data = ListItemData{
        creator: option.creator,
        owner: option.owner,
        collateral: option.collateral,
        counter_offer: option.counter_offer,
        expires: option.expires,
        price: item.price,
        list_expires: item.expires_at,
    };
    Ok(item_data)
}

pub fn query_list_item(deps: Deps,_env: Env) -> StdResult<ListItemsResponse>{
    let state = CONTRACT_PARAMS.load(deps.storage)?;
    let contrac_addr = state.option_address;
    let items:StdResult<Vec<_>> =LISTITEM_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let items =items?;
    let mut resp: Vec<(u64,ListItemData)> = Vec::new();
    for item in items{
        let option = get_option_info(deps, &contrac_addr, item.0).unwrap();
        if option.isBurned ||_env.block.time > option.expires || option.owner != item.1.seller {
            continue;
        }
        let item_data = ListItemData{
            creator: option.creator,
            owner: option.owner,
            collateral: option.collateral,
            counter_offer: option.counter_offer,
            expires: option.expires,
            price: item.1.price,
            list_expires: item.1.expires_at,
        };
        resp.push((item.0,item_data));
    }
    Ok(resp)
}

pub fn query_list_item_page(deps: Deps,_env: Env,key: u64,amount: u64) -> StdResult<ListItemsResponse>{
    let state = CONTRACT_PARAMS.load(deps.storage)?;
    let contrac_addr = state.option_address;
    let items:StdResult<Vec<_>> =LISTITEM_LIST.range(deps.storage, Some(Bound::exclusive(key)), None, Order::Ascending).collect();
    let items =items?;
    let mut resp: Vec<(u64,ListItemData)> = Vec::new();
    for item in items{
        let option = get_option_info(deps, &contrac_addr, item.0).unwrap();
        if option.isBurned ||_env.block.time > option.expires || option.owner != item.1.seller {
            continue;
        }
        let item_data = ListItemData{
            creator: option.creator,
            owner: option.owner,
            collateral: option.collateral,
            counter_offer: option.counter_offer,
            expires: option.expires,
            price: item.1.price,
            list_expires: item.1.expires_at,
        };
        resp.push((item.0,item_data));
        if resp.len() as u64 >= amount{
            break;
        }
    }
    Ok(resp)
}

pub fn query_owner_list_items(deps: Deps, addr: String) -> StdResult<ListItemsResponse>{
    let mut address = deps.api.addr_validate(&addr)?;
    let state = CONTRACT_PARAMS.load(deps.storage)?;
    let contrac_addr = state.option_address;
    let items:StdResult<Vec<_>> =LISTITEM_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let items =items?;
    let mut resp: Vec<(u64,ListItemData)> = Vec::new();
    for item in items{
        if item.1.seller != address{
            continue;
        }
        let option = get_option_info(deps, &contrac_addr, item.0).unwrap();
        let item_data = ListItemData{
            creator: option.creator,
            owner: option.owner,
            collateral: option.collateral,
            counter_offer: option.counter_offer,
            expires: option.expires,
            price: item.1.price,
            list_expires: item.1.expires_at,
        };
        resp.push((item.0,item_data));
    }
    Ok(resp)

}

pub fn query_owner_unlist_items(deps:Deps,addr: String) -> StdResult<ListItemsResponse>{
    let mut address = deps.api.addr_validate(&addr)?;
    let state = CONTRACT_PARAMS.load(deps.storage)?;
    let contrac_addr = state.option_address;
    let options = get_owner_options(deps, &contrac_addr, &address).unwrap();
    let mut resp: Vec<(u64,ListItemData)> = Vec::new();
    for option in options{
        if LISTITEM_LIST.has(deps.storage, option.0){
            continue;
        }
        let item_data = ListItemData{
            creator: option.1.creator,
            owner: option.1.owner,
            collateral: option.1.collateral,
            counter_offer: option.1.counter_offer,
            expires: option.1.expires,
            price: Uint128::new(0),
            list_expires: Timestamp::from_seconds(0),
        };
        resp.push((option.0,item_data));
    }
    Ok(resp)
}


fn get_option_info(
    deps: Deps,
    option_contract: &Addr,
    token_id: u64,
) -> Result<GetOptionByIdResponse, ContractError> {
    let query_msg: OPQueryMsg =OPQueryMsg::GetOptionByid {id: token_id};
    let query_response: GetOptionByIdResponse =
            deps.querier.query_wasm_smart(option_contract.clone(), &query_msg)?;
    return Ok(query_response);
}

fn get_owner_options(    deps: Deps,
    option_contract: &Addr,
    owner: &Addr,
) -> Result<OptionsResponse, ContractError>{
    let query_msg: OPQueryMsg =OPQueryMsg::OwnerOptions { addr: owner.to_string()};
    let query_response: OptionsResponse =
            deps.querier.query_wasm_smart(option_contract.clone(), &query_msg)?;
    return Ok(query_response);
}
