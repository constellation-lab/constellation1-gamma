use cosmwasm_std::{
    entry_point, to_json_binary, to_binary, Binary, Coin, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Uint128, SubMsg,
    };
    use cw_storage_plus::{Item, Map};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::state::{LiquidityPool, LiquidityProviderInfo, LP_INFO, LP_TOKENS, POOL};
    const CONSTELLA_OPTION_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";



    #[entry_point]
    pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
    let liquidity_pool = LiquidityPool {
    assets: vec![],
    lp_token_supply: Uint128::zero(),
    };

    POOL.save(deps.storage, &liquidity_pool)?;

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
ExecuteMsg::Deposit { assets } => execute_deposit(deps, env, info, assets),
ExecuteMsg::Withdraw { lp_tokens } => execute_withdraw(deps, env, info, lp_tokens),
ExecuteMsg::MintOption {
collateral,
counter_offer,
expiration,
} => execute_mint_option(deps, env, info, collateral, counter_offer, expiration),
ExecuteMsg::DistributePremium { option_id } => {
execute_distribute_premium(deps, env, info, option_id)
}
}
}
fn execute_deposit(
deps: DepsMut,
_env: Env,
info: MessageInfo,
assets: Vec<Coin>,
) -> Result<Response, ContractError> {
// Update liquidity pool assets
let mut pool = POOL.load(deps.storage)?;
pool.assets.extend(assets.clone());
POOL.save(deps.storage, &pool)?;

// Calculate and mint LP tokens
let lp_tokens = calculate_lp_tokens(&pool, &assets);
LP_TOKENS.update(deps.storage, info.sender.clone(), |balance| -> StdResult<_> {
    Ok(balance.unwrap_or_default() + lp_tokens)
})?;

// Update LP info
let lp_info = LiquidityProviderInfo {
    address: info.sender.clone(),
    assets: assets.clone(),
    lp_tokens,
};
LP_INFO.save(deps.storage, info.sender.clone(), &lp_info)?;

Ok(Response::default())

}
fn execute_withdraw(
deps: DepsMut,
_env: Env,
info: MessageInfo,
lp_tokens: Uint128,
) -> Result<Response, ContractError> {
// Check if LP has sufficient balance
let lp_balance = LP_TOKENS
.may_load(deps.storage, info.sender.clone())?
.unwrap_or_default();
if lp_balance < lp_tokens {
return Err(ContractError::InsufficientLPTokens {});
}

// Calculate and transfer assets to LP
let pool = POOL.load(deps.storage)?;
let assets = calculate_assets_for_lp_tokens(&pool, lp_tokens);
let transfer_msgs = assets
    .iter()
    .map(|coin| cosmwasm_std::BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![coin.clone()],
    })
    .collect::<Vec<_>>();

// Burn LP tokens
LP_TOKENS.update(deps.storage, info.sender.clone(), |balance| -> StdResult<_> {
    Ok(balance.unwrap_or_default().checked_sub(lp_tokens)?)
})?;

// Update LP info
let lp_info = LP_INFO.load(deps.storage, info.sender.clone())?;
let updated_lp_info = LiquidityProviderInfo {
    assets: lp_info
        .assets
        .into_iter()
        .map(|coin| {
            let asset_amount = assets
                .iter()
                .find(|&a| a.denom == coin.denom)
                .unwrap()
                .amount;
            Coin {
                denom: coin.denom,
                amount: coin.amount.checked_sub(asset_amount).unwrap(),
            }
        })
        .collect(),
    ..lp_info
};
LP_INFO.save(deps.storage, info.sender.clone(), &updated_lp_info)?;

Ok(Response::new().add_messages(transfer_msgs))

}
fn execute_mint_option(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collateral: Coin,
    counter_offer: Coin,
    expiration: u64,
    ) -> Result<Response, ContractError> {
    let constella_option_contract = deps.api.addr_validate(CONSTELLA_OPTION_CONTRACT)?;
    let mint_msg = constellation::msg::ExecuteMsg::Create {
    counter_offer: vec![counter_offer],
    time_stamp: expiration,
    };
    let mint_option_msg = cosmwasm_std::WasmMsg::Execute {
    contract_addr: constella_option_contract.into_string(),
    msg: to_binary(&mint_msg)?,
    funds: vec![],
    };

    Ok(Response::new().add_message(mint_option_msg))

}

fn execute_distribute_premium(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    option_id: u64,
    ) -> Result<Response, ContractError> {
    let constella_option_contract = deps.api.addr_validate(CONSTELLA_OPTION_CONTRACT)?;
    let premium_query = constellation::msg::QueryMsg::GetOptionByid { id: option_id };
    let premium_response: constellation::msg::GetOptionByIdResponse =
    deps.querier.query_wasm_smart(constella_option_contract, &premium_query)?;
    
    
// Distribute premium to LPs based on their share
let pool = POOL.load(deps.storage)?;
let total_lp_tokens = pool.lp_token_supply;
let premium_per_lp_token = premium_response.counter_offer.amount / total_lp_tokens;

let submessages: Vec<SubMsg> = LP_INFO
    .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    .map(|result| {
        let (_, lp_info) = result.unwrap();
        let lp_premium = premium_per_lp_token * lp_info.lp_tokens;
        let transfer_msg = cosmwasm_std::BankMsg::Send {
            to_address: lp_info.address.to_string(),
            amount: vec![Coin {
                denom: premium_response.counter_offer.denom.clone(),
                amount: lp_premium,
            }],
        };
        SubMsg::new(transfer_msg)
    })
    .collect();

Ok(Response::new().add_submessages(submessages))
}



#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
match msg {
QueryMsg::GetPool {} => to_binary(&query_pool(deps)?),
QueryMsg::GetLiquidityProviderInfo { address } => {
to_binary(&query_liquidity_provider_info(deps, address)?)
}
}
}
fn query_pool(deps: Deps) -> StdResult<LiquidityPool> {
POOL.load(deps.storage)
}
fn query_liquidity_provider_info(deps: Deps, address: String) -> StdResult<LiquidityProviderInfo> {
let addr = deps.api.addr_validate(&address)?;
LP_INFO.load(deps.storage, addr)
}

fn calculate_lp_tokens(pool: &LiquidityPool, assets: &[Coin]) -> Uint128 {
    if pool.assets.is_empty() {
    // If the pool is empty, the LP tokens minted will be equal to the total supply of assets
    assets.iter().map(|coin| coin.amount).sum()
    } else {
    // Calculate the share of the deposited assets in relation to the pool's total assets
    let total_pool_assets: Uint128 = pool.assets.iter().map(|coin| coin.amount).sum();
    let deposited_assets: Uint128 = assets.iter().map(|coin| coin.amount).sum();
    let share = Decimal::from_ratio(deposited_assets, total_pool_assets);

    // Calculate the amount of LP tokens to mint based on the share and the pool's total LP token supply
    let lp_tokens = share * Uint128::from(pool.lp_token_supply);
    lp_tokens
}
}
fn calculate_assets_for_lp_tokens(pool: &LiquidityPool, lp_tokens: Uint128) -> Vec<Coin> {
let total_lp_tokens = Uint128::from(pool.lp_token_supply);
let share = Decimal::from_ratio(lp_tokens, total_lp_tokens);
pool.assets
    .iter()
    .map(|coin| Coin {
        denom: coin.denom.clone(),
        amount: coin.amount * share,
    })
    .collect()

}

