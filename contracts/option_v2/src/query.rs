use cosmwasm_std::{Binary, Deps, Env, StdResult};
use crate::state::{CONFIG,OPTION_LIST, Data, APPROVE_LIST};
use crate::msg::{ConfigResponse, QueryMsg}; 
use crate::msg::{OptionsResponse, GetOptionByIdResponse};
use cosmwasm_std::{
    entry_point, to_json_binary,Order};
use cw_storage_plus::Bound;

//Queries
#[cfg_attr(not(feature = "library"), entry_point)]
#[allow(dead_code)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::Options {}=> to_json_binary(&query_options(deps)?),
        QueryMsg::OptionsPage { key, amount }=>to_json_binary(&query_options_page(deps, key, amount)?),
        QueryMsg::GetOptionByid { id }=>to_json_binary(&query_option_by_id(deps,id)?),
        QueryMsg::OwnerOptions { addr }=>to_json_binary(&query_owner_options(deps, addr)?),
        QueryMsg::CreateorOptions { addr }=>to_json_binary(&query_creator_options(deps, addr)?),
        QueryMsg::GetIsApprove { spender, owner } =>to_json_binary(&query_is_approve(deps,spender,owner)?)
    }
}

pub fn query_is_approve(deps:Deps,spender:String,owner:String)->StdResult<bool>{
    let mut spender_addr = deps.api.addr_validate(&spender)?;
    let mut onwer_addr = deps.api.addr_validate(&owner)?;

    let is_approve = match APPROVE_LIST.load(deps.storage,(onwer_addr,spender_addr)){
        Ok(is_approve)=>is_approve,
        Err(_)=> return Ok(false)
    };
    Ok(is_approve)
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



fn query_owner_options(deps: Deps,addr: String)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> = OPTION_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let mut address = deps.api.addr_validate(&addr)?;
    let mut resp: Vec<(u64,Data)> = Vec::new();
    for (id,option) in options?{
        if option.owner == address{
            resp.push((id,option));
        }
    }
    Ok(resp)
}

fn query_creator_options(deps: Deps,addr: String)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> = OPTION_LIST.range(deps.storage, None, None, Order::Ascending).collect();
    let mut address = deps.api.addr_validate(&addr)?;
    let mut resp: Vec<(u64,Data)> = Vec::new();
    for (id,option) in options?{
        if option.creator == address{
            resp.push((id,option));
        }
    }
    Ok(resp)
}