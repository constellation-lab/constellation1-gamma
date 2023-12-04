
use std::collections::HashMap;
use cosmwasm_std::{BankMsg, Uint128}; 
use crate::state::{Data, OptionStatus};
use crate::error::ContractError;
use crate::contracts::*;
use crate::operations::*;
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
        assert_eq!("creator", res.creator.as_str());
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
        execute(deps.as_mut(), mock_env(), mock_info("alice",&coins(0,"")), ExecuteMsg::Claim { id: 0 }).unwrap_err();
        let mut now_env = mock_env();
        now_env.block.time = Timestamp::from_seconds(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+5);
        let res = execute(deps.as_mut(), now_env, mock_info("alice",&coins(0,"")), ExecuteMsg::Claim { id: 0 }).unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(100, "ETH"),
            })
        );
    }
   
    #[test]
    fn place_bid_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Create an option
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

        // Place a bid
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(200_000); // set a time in the future to pass the time validation
        let res = execute_place_bid(
            deps.as_mut(),
            env,
            mock_info("bob", &coins(100, "BTC")),
            0,
            coins(100, "BTC"), // assuming the bid amount is the same as the counter_offer
        )
        .unwrap();

        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "place_bid");
    }
    
    
    

    #[test]
    fn place_offer_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option
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
    
        // Place an offer
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(200_000); // set a time in the future to pass the time validation
        let res = execute_place_offer(
            deps.as_mut(),
            env,
            mock_info("bob", &coins(100, "BTC")),
            0,
            coins(100, "BTC"), // assuming the offer amount is the same as the counter_offer
        )
        .unwrap();
    
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "place_offer");
    }
    
    #[test]
    fn accept_bid_or_offer_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option
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
    
        // Accept bid or offer
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(200_000); // set a time in the future to pass the time validation
        let res = execute_accept_bid_or_offer(deps.as_mut(), env, 0);
    
        match res {
            Ok(_) => {
                assert!(false, "Expected an error, but received Ok");
            }
            Err(error) => {
                if let ContractError::NoBidOrOffer = error {
                    // This is the expected error
                } else {
                    panic!("Unexpected error type: {:?}", error);
                }
            }
        }
    }
    


    #[test]
    fn execute_partial_test() {
        // Initialize mock dependencies
        let mut deps = mock_dependencies();
    
        // Instantiate the contract
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option
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
    
        // Execute partial
        let res = execute_partial(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            0.5,
        )
        .unwrap();
    
        // Validate the response
        assert_eq!(res.messages.len(), 2);
        
        // Validate the first message for partial collateral transfer
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(50, "ETH"),
            })
        );
    
        // Validate the second message for partial counter offer transfer
        assert_eq!(
            res.messages[1].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "alice".into(),
                amount: coins(50, "BTC"),
            })
        );
    }
    

    #[test]
    fn execute_buy_fraction_test() {
        // Initialize mock dependencies
        let mut deps = mock_dependencies();
    
        // Instantiate the contract
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();
    
        // Create an option in the market
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
    
        // Execute buy fraction
        let res = execute_buy_fraction(
            deps.as_mut(),
            mock_env(),
            mock_info("bob", &coins(100, "ETH")),
            0,
            0.5,
        );
        
        // Print or log the result for debugging
        println!("{:?}", res);

        // Validate the result
        assert!(res.is_ok());
    
        // Add additional assertions or checks if needed
    }
    
    #[test]
    fn extend_expiration_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), msg).unwrap();
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

        let res = execute_extend_expiration(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), 0, 11792624898);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn pause_and_unpause_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();

        let res_pause = execute_pause(deps.as_mut(), mock_info("admin", &coins(0, "")));
        assert!(res_pause.is_ok());

        let res_unpause = execute_unpause(deps.as_mut(), mock_info("admin", &coins(0, "")));
        assert!(res_unpause.is_ok());

        // Add assertions or additional checks if needed
    }


    #[test]
    fn add_oracle_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();
        let res = execute_add_oracle(deps.as_mut(), mock_info("admin", &coins(0, "")), Addr::unchecked("oracle"));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn update_price_oracle_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();
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

        // Update price oracle
        let res = execute_update_price_oracle(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), 0, coins(150, "ETH"));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    
    #[test]
    fn execute_set_option_parameters_test() {
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

        // Set option parameters
        let mut parameters = HashMap::new();
        parameters.insert("param1".to_string(), "value1".to_string());
        parameters.insert("param2".to_string(), "value2".to_string());

        let res = execute_set_option_parameters(
            deps.as_mut(),
            mock_info("creator", &coins(0, "")),
            0,
            parameters.clone(),
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_notify_option_expiry_test() {
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

        // Notify option expiry
        let res = execute_notify_option_expiry(deps.as_mut(), mock_env(), 0, 10);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_get_option_history_test() {
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

        // Get option history
        let res = execute_get_option_history(deps.as_mut(), 0);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
    
    #[test]
    fn execute_calculate_option_risk_metrics_test() {
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

        // Calculate option risk metrics
        let res = execute_calculate_option_risk_metrics(deps.as_mut(), 0);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_provide_liquidity_test() {
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

        // Provide liquidity
        let res = execute_provide_liquidity(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(100, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_withdraw_liquidity_test() {
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

        // Withdraw liquidity
        let res = execute_withdraw_liquidity(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
    
    
    #[test]
    fn execute_vote_on_governance_proposal_test() {
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

        // Vote on governance proposal
        let res = execute_vote_on_governance_proposal(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), 0, true);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_use_option_as_collateral_test() {
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

        // Use option as collateral
        let res = execute_use_option_as_collateral(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_wrap_option_for_yield_farming_test() {
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

        // Wrap option for yield farming
        let res = execute_wrap_option_for_yield_farming(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
        }
    
    
    #[test]
    fn execute_create_amm_pool_test() {
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

        // Create AMM pool
        let res = execute_create_amm_pool(deps.as_mut(), mock_env(), mock_info("alice", &coins(0, "")), 0);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_trade_on_amm_test() {
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

        // Trade on AMM
        let res = execute_trade_on_amm(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            coins(50, "ETH"),  // Use `coins` instead of `vec!`
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_integrate_market_data_feed_test() {
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

        // Integrate market data feed
        let res = execute_integrate_market_data_feed(
            deps.as_mut(),
            mock_env(),
            mock_info("alice", &coins(0, "")),
            0,
            "https://example.com/data-feed".to_string(),
        );
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
        
    #[test]
    fn execute_refer_user_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Refer user
        let res = execute_refer_user(deps.as_mut(), mock_info("alice", &coins(0, "")), Addr::unchecked("bob"));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_set_discount_criteria_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Set discount criteria
        let mut criteria = HashMap::new();
        criteria.insert("param1".to_string(), "value1".to_string());
        criteria.insert("param2".to_string(), "value2".to_string());

        let res = execute_set_discount_criteria(deps.as_mut(), mock_info("alice", &coins(0, "")), criteria);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }
        
    /*

    //Test for creating tokens eg ERC 20 tokens and locking tokens - to be implemented
    #[test]
    fn execute_create_token_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("admin", &coins(0, "")), msg).unwrap();

        // Create token
        let res = execute_create_token(deps.as_mut(), mock_info("admin", &coins(0, "")), Coin {
            denom: "usdc".to_string(),
            amount: "100".to_string(),
        }, Timestamp::from_seconds(11692624898));
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

    #[test]
    fn execute_lock_tokens_test() {
        let mut deps = mock_dependencies();
        let msg: InstantiateMsg = InstantiateMsg {};
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &coins(0, "")), msg).unwrap();

        // Lock tokens
        let res = execute_lock_tokens(deps.as_mut(), mock_env(), mock_info("alice", &coins(100, "TOKEN")), vec![coin(100, "TOKEN")], 10);
        assert!(res.is_ok());

        // Add assertions or additional checks if needed
    }

*/
}