#[cfg(test)]
mod tests {
use super::*;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Addr, Uint128};

#[test]
fn test_deposit() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(0, ""));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info("user1", &coins(100, "uatom"));
    let assets = vec![Coin::new(100, "uatom")];
    let msg = ExecuteMsg::Deposit { assets };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(1, res.events.len());

    let event = &res.events[0];
    assert_eq!("deposit", event.ty);
    assert_eq!(
        &[
            attr("action", "deposit"),
            attr("lp_address", "user1"),
            attr("assets", "[\"100uatom\"]"),
            attr("lp_tokens", "100"),
        ],
        event.attributes.as_slice()
    );

    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetPool {}).unwrap();
    let pool_res: LiquidityPool = from_binary(&query_res).unwrap();
    assert_eq!(1, pool_res.assets.len());
    assert_eq!(Uint128::new(100), pool_res.assets[0].amount);
    assert_eq!("uatom", pool_res.assets[0].denom);
    assert_eq!(Uint128::new(100), pool_res.lp_token_supply);

    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetLiquidityProviderInfo {
            address: "user1".to_string(),
        },
    )
    .unwrap();
    let lp_info_res: LiquidityProviderInfo = from_binary(&query_res).unwrap();
    assert_eq!(Addr::unchecked("user1"), lp_info_res.address);
    assert_eq!(1, lp_info_res.assets.len());
    assert_eq!(Uint128::new(100), lp_info_res.assets[0].amount);
    assert_eq!("uatom", lp_info_res.assets[0].denom);
    assert_eq!(Uint128::new(100), lp_info_res.lp_tokens);
}

#[test]
fn test_withdraw() {
    // Test initialization and deposit
    // ...

    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::Withdraw {
        lp_tokens: Uint128::new(50),
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(1, res.events.len());

    let event = &res.events[0];
    assert_eq!("withdraw", event.ty);
    assert_eq!(
        &[
            attr("action", "withdraw"),
            attr("lp_address", "user1"),
            attr("assets", "[\"50uatom\"]"),
            attr("lp_tokens", "50"),
        ],
        event.attributes.as_slice()
    );

    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetPool {}).unwrap();
    let pool_res: LiquidityPool = from_binary(&query_res).unwrap();
    assert_eq!(1, pool_res.assets.len());
    assert_eq!(Uint128::new(50), pool_res.assets[0].amount);
    assert_eq!("uatom", pool_res.assets[0].denom);
    assert_eq!(Uint128::new(50), pool_res.lp_token_supply);

    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetLiquidityProviderInfo {
            address: "user1".to_string(),
        },
    )
    .unwrap();
    let lp_info_res: LiquidityProviderInfo = from_binary(&query_res).unwrap();
    assert_eq!(Addr::unchecked("user1"), lp_info_res.address);
    assert_eq!(1, lp_info_res.assets.len());
    assert_eq!(Uint128::new(50), lp_info_res.assets[0].amount);
    assert_eq!("uatom", lp_info_res.assets[0].denom);
    assert_eq!(Uint128::new(50), lp_info_res.lp_tokens);
}

// Add more test functions for other messages and queries
// ...

}

#[test]
fn test_mint_option() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(0, ""));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Deposit assets into the liquidity pool
    let info = mock_info("user1", &coins(100, "uatom"));
    let assets = vec![Coin::new(100, "uatom")];
    let msg = ExecuteMsg::Deposit { assets };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Mint an option
    let info = mock_info("user1", &[]);
    let collateral = Coin::new(50, "uatom");
    let counter_offer = Coin::new(100, "uosmo");
    let expiration = 1000;
    let msg = ExecuteMsg::MintOption {
        collateral,
        counter_offer,
        expiration,
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(1, res.events.len());

    let event = &res.events[0];
    assert_eq!("option_minted", event.ty);
    assert_eq!(
        &[
            attr("action", "option_minted"),
            attr("option_id", "1"),
            attr("collateral", "50uatom"),
            attr("counter_offer", "100uosmo"),
            attr("expiration", "1000"),
        ],
        event.attributes.as_slice()
    );

    // Query the minted option
    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetOptionById { option_id: 1 }).unwrap();
    let option_res: Option = from_binary(&query_res).unwrap();
    assert_eq!(collateral, option_res.collateral);
    assert_eq!(counter_offer, option_res.counter_offer);
    assert_eq!(expiration, option_res.expiration);
}

#[test]
fn test_distribute_premium() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(0, ""));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Deposit assets into the liquidity pool
    let info = mock_info("user1", &coins(100, "uatom"));
    let assets = vec![Coin::new(100, "uatom")];
    let msg = ExecuteMsg::Deposit { assets };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Mint an option
    let info = mock_info("user1", &[]);
    let collateral = Coin::new(50, "uatom");
    let counter_offer = Coin::new(100, "uosmo");
    let expiration = 1000;
    let msg = ExecuteMsg::MintOption {
        collateral,
        counter_offer,
        expiration,
    };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Distribute premium
    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::DistributePremium { option_id: 1 };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(1, res.events.len());

    let event = &res.events[0];
    assert_eq!("premium_distributed", event.ty);
    assert_eq!(
        &[
            attr("action", "premium_distributed"),
            attr("option_id", "1"),
            attr("premium", "10uosmo"),
        ],
        event.attributes.as_slice()
    );

    // Query the updated liquidity provider info
    let query_res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetLiquidityProviderInfo {
            address: "user1".to_string(),
        },
    )
    .unwrap();
    let lp_info_res: LiquidityProviderInfo = from_binary(&query_res).unwrap();
    assert_eq!(Uint128::new(110), lp_info_res.lp_tokens);
}

