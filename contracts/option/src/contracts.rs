use cosmwasm_std::{
    entry_point,to_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin,Timestamp,Order,Binary
};
use cw_storage_plus::Bound;
use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, OptionsResponse, GetOptionByIdResponse};
use crate::state::{State, CONFIG, Data,OPTION_LIST,CREATOR_LIST,OWNER_LIST,MARKET_LIST};

// like solidity's constractor func, just run once when the contract init.
#[entry_point]
#[allow(dead_code)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        creator: info.sender.clone(),
        total_options_num: 0,
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
    OPTION_LIST.save(deps.storage,id , &option)?;
    CREATOR_LIST.save(deps.storage, (info.sender.clone(),id), &option)?;
    OWNER_LIST.save(deps.storage,(info.sender.clone(),id), &option)?;

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
    let old_owner = option.owner;
    option.owner = info.sender.clone();
    option.price = Vec::new();
    option.onsale = false;
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
    if option.expires<env.block.time{
        return Err(ContractError::OptionNotExpired { expires: option.expires });
    }
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send { to_address: option.creator.to_string(), amount: option.collateral}).add_attribute("action", "claim");

    OPTION_LIST.remove(deps.storage, id);
    OWNER_LIST.remove(deps.storage,(option.owner,id));
    CREATOR_LIST.remove(deps.storage,(option.creator,id));
    if MARKET_LIST.has(deps.storage, id){
        MARKET_LIST.remove(deps.storage, id);
    }
    res = res.add_attribute("action", "claim");
    Ok(res)
}


pub fn execute_create(
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
    let new_data:Data = Data { creator: info.sender.clone(), owner: info.sender.clone(), collateral: info.funds, counter_offer: counter_offer, onsale: false, expires:expires ,price: Vec::new() };
    let mut state = CONFIG.load(deps.storage)?;
    //save the key id to the own and creator's list
    let key: u64 = state.total_options_num;
    OPTION_LIST.save(deps.storage,state.total_options_num , &new_data)?;
    CREATOR_LIST.save(deps.storage, (info.sender.clone(),key), &new_data)?;
    OWNER_LIST.save(deps.storage,(info.sender.clone(),key), &new_data)?;

    //save the total_option+1
    state.total_options_num=key + 1;
    CONFIG.save(deps.storage, &state)?;
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
    OPTION_LIST.save(deps.storage, id, &option)?;
    MARKET_LIST.remove(deps.storage, id);
    OWNER_LIST.save(deps.storage, (option.owner.clone(),id), &option)?;
    CREATOR_LIST.save(deps.storage, (option.creator.clone(),id), &option)?;

    let res = Response::new().add_attribute("action", "remove form market");
    Ok(res)
}

#[entry_point]
#[allow(dead_code)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Options {}=> to_binary(&query_options(deps)?),
        QueryMsg::OptionsPage { key, amount }=>to_binary(&query_options_page(deps, key, amount)?),
        QueryMsg::GetOptionByid { id }=>to_binary(&query_option_by_id(deps,id)?),
        QueryMsg::MarketOptions {} =>to_binary(&query_market_options(deps)?),
        QueryMsg::MaketOptionsPagee { key, amount }=>to_binary(&query_market_options_page(deps, key, amount)?),
        QueryMsg::OwnerOptions { addr }=>to_binary(&query_owner_options(deps, addr)?),
        QueryMsg::CreateorOptions { addr }=>to_binary(&query_creator_options(deps, addr)?),
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

fn query_options_page(deps: Deps,key: u64,amount:usize)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> =OPTION_LIST.range(deps.storage, Some(Bound::exclusive(key)), None, Order::Ascending).take(amount).collect();
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

fn query_market_options_page(deps: Deps,key: u64,amount:usize)->StdResult<OptionsResponse>{
    let options:StdResult<Vec<_>> =MARKET_LIST.range(deps.storage, Some(Bound::exclusive(key)), None, Order::Ascending).take(amount).collect();
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
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};
    #[test]
    fn proper_initialization(){
        let mut deps = mock_dependencies();

        let msg: InstantiateMsg = InstantiateMsg {};
        let info = mock_info("creator", &coins(0, ""));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!("creator", res.creator.as_str());
        assert_eq!(0,res.total_options_num);
    }
}