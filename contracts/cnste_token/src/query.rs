use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::msg::{TokenInfoResponse, StakeInfo as StakeInfoResponse};
use crate::state::{CONFIG, STAKE_INFO};

pub fn balance(deps: Deps, address: String) -> StdResult<Uint128> {
    let address = deps.api.addr_validate(&address)?;
    Ok(cw20_base::contract::query_balance(deps, address.to_string())?.balance)
}

pub fn token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let info = cw20_base::contract::query_token_info(deps)?;
    Ok(TokenInfoResponse {
        name: info.name,
        symbol: info.symbol,
        decimals: info.decimals,
        total_supply: info.total_supply,
    })
}

pub fn stake_info(deps: Deps, address: String) -> StdResult<StakeInfoResponse> {
    let address = deps.api.addr_validate(&address)?;
    let info = STAKE_INFO.may_load(deps.storage, address)?;
    let staked_amount = info.map(|i| i.staked_amount).unwrap_or_default();
    Ok(StakeInfoResponse { staked_amount })
}



/*pub fn query_stake_info(deps: Deps, address: String) -> StdResult<StakeInfo> {
    let address = deps.api.addr_validate(&address)?;
    let stake_info = STAKE_INFO.load(deps.storage, &address).unwrap_or_default();
    Ok(stake_info)
}*/