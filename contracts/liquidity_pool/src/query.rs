use cosmwasm_std::{Binary, Deps, Env, StdResult, to_binary};
use crate::msg::{QueryMsg};
use crate::state::{POOL, LP_INFO, LiquidityPool, LiquidityProviderInfo};
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
match msg {
QueryMsg::GetPool {} => to_binary(&query_pool(deps)?),
QueryMsg::GetLiquidityProviderInfo { address } => {
to_binary(&query_liquidity_provider_info(deps, address)?)
}
// Add more query handlers as needed
}
}
pub fn query_pool(deps: Deps) -> StdResult<LiquidityPool> {
POOL.load(deps.storage)
}
pub fn query_liquidity_provider_info(deps: Deps, address: String) -> StdResult<LiquidityProviderInfo> {
let addr = deps.api.addr_validate(&address)?;
LP_INFO.load(deps.storage, addr)
}