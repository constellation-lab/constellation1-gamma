use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Coin, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Option, OPTIONS};
use crate::events::OptionMarketplaceEvent;
use crate::query::{query_option, query_options};
use crate::msg::PricingOracleQueryMsg;
use constellation::state::OPTION_LIST;


const PRICING_ORACLE_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const CONSTELLA_OPTION_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const LIQUIDITY_POOL_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        Ok(Response::default())
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ListOption {
            option_id,
            price,
            slippage_tolerance,
            min_trade_amount,
        } => execute_list_option(deps, env, info, option_id, price, slippage_tolerance, min_trade_amount),
        ExecuteMsg::BuyOption {
            option_id,
            amount,
        } => execute_buy_option(deps, env, info, option_id, amount),
        ExecuteMsg::ExecuteOption { option_id } => execute_execute_option(deps, env, info, option_id),
    }
}


fn execute_list_option(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    option_id: u64,
    price: Uint128,
    slippage_tolerance: Decimal,
    min_trade_amount: Uint128,
) -> Result<Response, ContractError> {
    let option = OPTIONS.may_load(deps.storage, option_id)?;
    if option.is_none() {
        return Err(ContractError::OptionNotFound {});
    }
    let mut option = option.unwrap();

    if option.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if option.is_listed {
        return Err(ContractError::OptionAlreadyListed {});
    }

    let pricing_oracle_contract = deps.api.addr_validate(PRICING_ORACLE_CONTRACT)?;

    //let pricing_oracle_contract = PRICING_ORACLE_CONTRACT.load(deps.storage)?;
    let calculated_price: Uint128 = deps.querier.query_wasm_smart(
        pricing_oracle_contract,
        &QueryMsg::GetOptionPrice {
            option_id,
            slippage_tolerance,
        },
    )?;

    if price < calculated_price {
        return Err(ContractError::InvalidPrice {});
    }

    option.is_listed = true;
    option.price = price;
    option.min_trade_amount = min_trade_amount;

    OPTIONS.save(deps.storage, option_id, &option)?;

    let event = OptionMarketplaceEvent::OptionListed {
        option_id,
        price,
        min_trade_amount,
    };
    
    Ok(Response::default().add_event(event.into_event()?))
}

fn execute_buy_option(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    option_id: u64,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let option = OPTIONS.may_load(deps.storage, option_id)?;
    if option.is_none() {
        return Err(ContractError::OptionNotFound {});
    }
    let option = option.unwrap();

    if !option.is_listed {
        return Err(ContractError::OptionNotListed {});
    }

    if amount < option.min_trade_amount {
        return Err(ContractError::InsufficientAmount {});
    }

    let total_price = option.price * amount;

    if info.funds.iter().find(|c| c.denom == option.price_denom && c.amount >= total_price).is_none() {
        return Err(ContractError::InsufficientFunds {});
    }

    let mut option_mut = option.clone();
    option_mut.is_listed = false;
    option_mut.buyer = info.sender.clone();
    option_mut.amount = amount;

    OPTIONS.save(deps.storage, option_id, &option_mut)?;

    let transfer_premium_msg = cosmwasm_std::BankMsg::Send {
        to_address: option.owner.to_string(),
        amount: vec![Coin {
            denom: option.price_denom.clone(),
            amount: total_price,
        }],
    };

    let event = OptionMarketplaceEvent::OptionBought {
        option_id,
        buyer: info.sender.clone(),
        amount,
        total_price,
    };

    Ok(Response::default().add_event(event.into_event()?))
    /*Ok(Response::new()
        .add_message(transfer_premium_msg)
        .add_event(event.into_event())
        .add_attribute("action", "buy_option")
        .add_attribute("option_id", option_id.to_string())
        .add_attribute("buyer", info.sender.to_string())
        .add_attribute("amount", amount.to_string())
        .add_attribute("total_price", total_price.to_string()))*/
}

fn execute_execute_option(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    option_id: u64,
) -> Result<Response, ContractError> {
    let option = OPTIONS.may_load(deps.storage, option_id)?;
    if option.is_none() {
        return Err(ContractError::OptionNotFound {});
    }
    let option = option.unwrap();

    if option.buyer == Addr::unchecked("") {
        return Err(ContractError::OptionNotBought {});
    }

    if option.buyer != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let constella_option_contract = deps.api.addr_validate(CONSTELLA_OPTION_CONTRACT)?;
    //let constella_option_contract = CONSTELLA_OPTION_CONTRACT.load(deps.storage)?;
    //let execute_msg = constella_option::msg::ExecuteMsg::ExecuteOption { option_id };
    let execute_msg = crate::msg::ExecuteMsg::ExecuteOption { option_id };
    let execute_option_msg = cosmwasm_std::WasmMsg::Execute {
        contract_addr: constella_option_contract.to_string(),
        msg: to_binary(&execute_msg)?,
        funds: vec![],
    };

    OPTIONS.remove(deps.storage, option_id);

    let event = OptionMarketplaceEvent::OptionExecuted { option_id };

    Ok(Response::default().add_event(event.into_event()?))
    /*Ok(Response::new()
        .add_message(execute_option_msg)
        .add_event(event.into_event())
        .add_attribute("action", "execute_option")
        .add_attribute("option_id", option_id.to_string())
        .add_attribute("executor", info.sender.to_string()))*/
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOption { option_id } => to_binary(&query_option(deps, option_id)?),
        QueryMsg::ListOptions {} => to_binary(&query_options(deps)?),
        QueryMsg::GetOptionPrice { option_id, slippage_tolerance } => {
            to_binary(&query_option_price(deps, option_id, slippage_tolerance)?)
        }
    }
}

fn query_option_price(deps: Deps, option_id: u64, slippage_tolerance: Decimal) -> StdResult<Uint128> {
    let option = OPTION_LIST.load(deps.storage, option_id)?;
    let pricing_oracle_contract = deps.api.addr_validate(PRICING_ORACLE_CONTRACT)?;

    let calculated_price: Uint128 = deps.querier.query_wasm_smart(
        pricing_oracle_contract,
        &PricingOracleQueryMsg::GetOptionPrice {
            option_id,
            slippage_tolerance,
        },
    )?;

    Ok(calculated_price)
}