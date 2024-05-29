use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::msg::{TokenInfoResponse, StakeInfo};
use crate::state::{CONFIG, STAKE_INFO};

pub fn balance(deps: Deps, address: String) -> StdResult<Uint128> {
    let address = deps.api.addr_validate(&address)?;
    Ok(cw20_base::contract::query_balance(deps, address.to_string())?.balance)
}

pub fn token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let info = CONFIG.load(deps.storage)?;
    let res = TokenInfoResponse {
        name: info.name,
        symbol: info.symbol,
        decimals: info.decimals,
        total_supply: info.total_supply,
    };
    Ok(res)
}

pub fn stake_info(deps: Deps, address: String) -> StdResult<StakeInfo> {
    let address = deps.api.addr_validate(&address)?;
    let info = STAKE_INFO.may_load(deps.storage, address)?;
    let staked_amount = info.map(|i| i.staked_amount).unwrap_or_default();
    Ok(StakeInfo { staked_amount })
}

pub fn voting_power(deps: Deps, address: String) -> StdResult<Uint128> {
    let address = deps.api.addr_validate(&address)?;
    Ok(cw20_base::contract::query_balance(deps, address.to_string())?.balance)
}