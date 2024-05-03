use cosmwasm_std::{Deps, StdResult, StdError};
use crate::state::{Option, OPTIONS};
use crate::msg::{OptionResponse, OptionsResponse};

pub fn query_option(deps: Deps, option_id: u64) -> StdResult<OptionResponse> {
    let option = OPTIONS.may_load(deps.storage, option_id)?;
    option.map(|opt| opt.into()).ok_or_else(|| StdError::not_found("Option not found"))
}

pub fn query_options(deps: Deps) -> StdResult<OptionsResponse> {
    let options = OPTIONS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|res| res.map(|(_, opt)| opt.into()))
        .collect::<StdResult<Vec<_>>>()?;
    Ok(OptionsResponse { options })
}