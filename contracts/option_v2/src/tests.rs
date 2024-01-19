
use std::collections::HashMap;
use cosmwasm_std::{BankMsg, Uint128}; 
use crate::error::ContractError;
use crate::contracts::*;
use crate::query::*;
use crate::msg::{ ExecuteMsg, InstantiateMsg}; 


//Tests
//Note: Tests for functions with placeholder logic may not pass till full implementation

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, Timestamp, CosmosMsg};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        let info = mock_info("creator", &coins(0, ""));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!("creator", res.owner.as_str());
        assert_eq!(0, res.total_options_num);

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-2", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!(2, res.total_options_num);
    }

    #[test]
    fn create_and_query_options() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(0, "")),
            msg,
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-2", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();

        let res = query_options(deps.as_ref()).unwrap();
        let aim_data = vec![
            (
                0,
                Data {
                    creator: Addr::unchecked("creator-1".to_string()),
                    owner: Addr::unchecked("creator-1".to_string()),
                    collateral: coins(100, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
            (
                1,
                Data {
                    creator: Addr::unchecked("creator-2".to_string()),
                    owner: Addr::unchecked("creator-2".to_string()),
                    collateral: coins(100, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
        ];
        assert_eq!(aim_data, res);

        let wrong_data = vec![
            (
                0,
                Data {
                    creator: Addr::unchecked("creator-1".to_string()),
                    owner: Addr::unchecked("creator-1".to_string()),
                    collateral: coins(100, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
            (
                1,
                Data {
                    creator: Addr::unchecked("creator-1".to_string()),
                    owner: Addr::unchecked("creator-1".to_string()),
                    collateral: coins(90, "ETH"),
                    counter_offer: coins(100, "BTC"),
                    onsale: false,
                    expires: Timestamp::from_seconds(11692624898),
                    price: vec![],
                    highest_bidder: None,
                    best_offer: None,
                    bid_history: None,
                    status: OptionStatus::Active,
                    parameters: None,
                    exercise_conditions: None,
                    history: vec![],
                    risk_metrics: None,
                    pool_share: Uint128::new(0),
                },
            ),
        ];
        assert_ne!(wrong_data, res);
    }

    #[test]
    fn transfer() -> Result<(), String> {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        let res = query_options(deps.as_ref()).unwrap();
        let aim_data = vec![(
            0,
            Data {
                creator: Addr::unchecked("creator-1".to_string()),
                owner: Addr::unchecked("creator-1".to_string()),
                collateral: coins(100, "ETH"),
                counter_offer: coins(100, "BTC"),
                onsale: false,
                expires: Timestamp::from_seconds(11692624898),
                price: Vec::new(),
                highest_bidder: None,
                best_offer: None,
                bid_history: None,
                status: OptionStatus::Active,
                parameters: None,
                exercise_conditions: None,
                history: Vec::new(),
                risk_metrics: None,
                pool_share: Uint128::new(0),
            },
        )];
        assert_eq!(aim_data, res);
    
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Transfer {
                id: 0,
                to: "creator-2".to_string(),
            },
        )
        .unwrap();
    
        let res = query_options(deps.as_ref()).unwrap();
        let aim_data = vec![(
            0,
            Data {
                creator: Addr::unchecked("creator-1".to_string()),
                owner: Addr::unchecked("creator-2".to_string()),
                collateral: coins(100, "ETH"),
                counter_offer: coins(100, "BTC"),
                onsale: false,
                expires: Timestamp::from_seconds(11692624898),
                price: Vec::new(),
                highest_bidder: None,
                best_offer: None,
                bid_history: None,
                status: OptionStatus::Active,
                parameters: None,
                exercise_conditions: None,
                history: Vec::new(),
                risk_metrics: None,
                pool_share: Uint128::new(0),
            },
        )];
        assert_eq!(aim_data, res);
    
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator-1", &coins(100, "ETH")),
            ExecuteMsg::Transfer {
                id: 0,
                to: "creator-2".to_string(),
            },
        );
    
        match res {
            Ok(_) => Err("validate the auth wrong".to_string()),
            Err(error) => {
                if let ContractError::Unauthorized {} = error {
                    Ok(())
                } else {
                    Err("wrong error type".to_string())
                }
            }
        }
    }
    
    #[test]
    fn market_action() -> Result<(), String> {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("bob", &coins(100, "ETH")),
            ExecuteMsg::Create {
                counter_offer: coins(100, "BTC"),
                time_stamp: 11692624898,
            },
        )
        .unwrap();
    
        // test add to market
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::AddToMarket {
                id: 0,
                amount: 100,
                denom: "usdc".to_string(),
            },
        )
        .unwrap();
    
        let res = query_market_options(deps.as_ref()).unwrap();
        let res_fr_options = query_option_by_id(deps.as_ref(), 0).unwrap();
        let aim_data = vec![(
            0,
            Data {
                creator: Addr::unchecked("alice".to_string()),
                owner: Addr::unchecked("alice".to_string()),
                collateral: coins(100, "ETH"),
                counter_offer: coins(100, "BTC"),
                onsale: true,
                price: coins(100, "usdc"),
                expires: Timestamp::from_seconds(11692624898),
                highest_bidder: None,
                best_offer: None,
                bid_history: None,
                status: OptionStatus::Active,
                parameters: None,
                exercise_conditions: None,
                history: Vec::new(),
                risk_metrics: None,
                pool_share: Uint128::new(0),
            },
        )];
        assert_eq!(aim_data, res);
        assert_eq!(aim_data[0].1, res_fr_options);
    
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("bob", &coins(100, "ETH")),
            ExecuteMsg::AddToMarket {
                id: 0,
                amount: 100,
                denom: "usdc".to_string(),
            },
        );
    
        let _ = match res {
            Ok(_) => Err::<(), String>("validate the auth wrong".to_string()),
            Err(error) => {
                if let ContractError::Unauthorized {} = error {
                    return Ok(());
                } else {
                    return Err("wrong error type".to_string());
                }
            }
        };
    
        // test remove
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::RemoveFromMarket { id: 0 },
        )
        .unwrap();
    
        let res = query_market_options(deps.as_ref()).unwrap();
        let empty_vec: Vec<(u64, Data)> = Vec::new();
        assert_eq!(res, empty_vec);
    
        // test buy and update price
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(100, "ETH")),
            ExecuteMsg::AddToMarket {
                id: 0,
                amount: 100,
                denom: "usdc".to_string(),
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            ExecuteMsg::UpdatePrice { id: 0, price: coins(120, "usdc") },
        )
        .unwrap();
    
        let res = query_option_by_id(deps.as_ref(), 0).unwrap();
        let aim_data = Data {
            creator: Addr::unchecked("alice".to_string()),
            owner: Addr::unchecked("alice".to_string()),
            collateral: coins(100, "ETH"),
            counter_offer: coins(100, "BTC"),
            onsale: true,
            price: coins(120, "usdc"),
            expires: Timestamp::from_seconds(11692624898),
            highest_bidder: None,
            best_offer: None,
            bid_history: None,
            status: OptionStatus::Active,
            parameters: None,
            exercise_conditions: None,
            history: Vec::new(),
            risk_metrics: None,
            pool_share: Uint128::new(0),
        };
        assert_eq!(res, aim_data);
    
        execute(deps.as_mut(), mock_env(), mock_info("bob", &coins(120, "usdc")), ExecuteMsg::Buy { id: 0 }).unwrap();
        let res = query_option_by_id(deps.as_ref(), 0).unwrap();
        let aim_data = Data {
            creator: Addr::unchecked("alice".to_string()),
            owner: Addr::unchecked("bob".to_string()),
            collateral: coins(100, "ETH"),
            counter_offer: coins(100, "BTC"),
            onsale: false,
            expires: Timestamp::from_seconds(11692624898),
            price: Vec::new(),
            highest_bidder: None,
            best_offer: None,
            bid_history: None,
            status: OptionStatus::Active,
            parameters: None,
            exercise_conditions: None,
            history: Vec::new(),
            risk_metrics: None,
            pool_share: Uint128::new(0),
        };
        assert_eq!(res, aim_data);
    
        Ok(())
    }
    
    
 #[test]
    fn burn(){
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: 11692624898 }).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("bob",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: 11692624898 }).unwrap();

        // expired returns funds
        let mut env = mock_env();
        env.block.height = 200_000;
        let res = execute_burn(deps.as_mut(), mock_info("alice", &coins(0, "")),0).unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "ETH"),
            })
        );

        // check deleted
        let _ = query_option_by_id(deps.as_ref(),0).unwrap_err();
    }

    #[test]
    fn execute_option(){
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: 11692624898 }).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), ExecuteMsg::Transfer { id: 0, to: "bob".to_string() }).unwrap();
        let res = execute(deps.as_mut(), mock_env(), mock_info("bob", &coins(100, "BTC")), ExecuteMsg::Execute { id: 0 }).unwrap();
        assert_eq!(res.messages.len(), 2);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "BTC"),
            })
        );
        assert_eq!(
            res.messages[1].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "bob".into(),
                amount: coins(100, "ETH"),
            })
        );
    }
    #[test]
    fn cliam(){
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(100, "ETH")), 
            ExecuteMsg::Create { counter_offer: coins(100, "BTC"), time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+2 }).unwrap();
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(0,"")), ExecuteMsg::ClaimCollateral{ id: 0 }).unwrap_err();
        let mut now_env = mock_env();
        now_env.block.time = Timestamp::from_seconds(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+5);
        let res = execute(deps.as_mut(), now_env, mock_info("alice",&coins(0,"")), ExecuteMsg::ClaimCollateral { id: 0 }).unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "ETH"),
            })
        );
    }
   
    

}