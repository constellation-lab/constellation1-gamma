
use cosmwasm_std::{Uint128, Addr, QueryRequest, WasmQuery, Decimal, WasmMsg, CosmosMsg};
#[allow(unused_imports)]
use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin,Timestamp};
use crate::state::{CONFIG, OPTION_LIST, MARKET_LIST, DISCOUNTS};
use crate::msg::{QueryMsg, GovernanceMsg, CollateralMsg, YieldFarmingMsg, AmmMsg, DataFeedMsg}; 
use std::collections::HashMap;
use crate::error::ContractError;
//use maplit::hashmap;  //use serde_json::to_string;
use crate::events::ConstellationDerivativeEvent;

pub fn execute_extend_expiration(deps: DepsMut, env: Env, info: MessageInfo, id: u64, new_expiration: u64) -> Result<Response, ContractError> {
    // Load the existing option from storage
    let mut option = match OPTION_LIST.load(deps.storage, id) {
        Ok(option) => option,
        Err(_) => return Err(ContractError::OptionNotFound {}),
    };

    // Validate the sender
    if info.sender != option.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Validate the time and new expiration
    if env.block.time >= option.expires 
   || Timestamp::from_seconds(new_expiration) <= env.block.time 
    {
        return Err(ContractError::InvalidExpiration {});
    }

    // Update the state with the new expiration time
    option.expires = Timestamp::from_seconds(new_expiration);

    ConstellationDerivativeEvent::emit_expiration_extended(deps.as_ref(), id, new_expiration)?;
    

    OPTION_LIST.save(deps.storage, id, &option)?;

    let res: Response = Response::new().add_attributes([("action", "extend_expiration")]);
    Ok(res)
}


pub fn execute_pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate the sender
    if info.sender != CONFIG.load(deps.storage)?.creator {
        return Err(ContractError::Unauthorized {});
    }

    // Update the paused state
    CONFIG.update(deps.storage, |mut state| {
        state.paused = true;
        Ok::<_, cosmwasm_std::StdError>(state)
    })?;

    ConstellationDerivativeEvent::emit_contract_paused(deps.as_ref())?;
    

    let res: Response = Response::new().add_attributes([("action", "pause")]);
    
    Ok(res)
}

pub fn execute_unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate the sender
    if info.sender != CONFIG.load(deps.storage)?.creator {
        return Err(ContractError::Unauthorized {});
    }

    // Update the paused state
    CONFIG.update(deps.storage, |mut state| {
        state.paused = false;
        Ok::<_, cosmwasm_std::StdError>(state)
    })?;

    ConstellationDerivativeEvent::emit_contract_unpaused(deps.as_ref())?;

    let res: Response = Response::new().add_attributes([("action", "unpause")]);

    Ok(res)
    }

    pub fn execute_add_oracle(deps: DepsMut, info: MessageInfo, oracle: Addr) -> Result<Response, ContractError> {
        // Validate the sender
        if info.sender != CONFIG.load(deps.storage)?.creator {
            return Err(ContractError::Unauthorized {});
        }
    
        // Add the oracle address to the state
        CONFIG.update(deps.storage, |mut state| {
            state.oracle = Some(oracle.clone());
            Ok::<_, cosmwasm_std::StdError>(state)
        })?;
    
        ConstellationDerivativeEvent::emit_oracle_added(deps.as_ref(),  info.sender.clone(), oracle.clone())?;

        let res: Response = Response::new().add_attributes([("action", "add_oracle"), ("oracle", oracle.as_str())]);
        
        Ok(res)
    }

    pub fn  execute_update_price_oracle( deps: DepsMut, env: Env, _info: MessageInfo, id: u64, price: Vec<Coin>) -> Result<Response, ContractError> {
        // Load the existing option from storage
        let mut option = match MARKET_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(_) => return Err(ContractError::OptionCanotFindInTheMarket {}),
        };

        // Validate the time
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Validate that an oracle is set
        let oracle_address = match CONFIG.load(deps.storage) {
            Ok(state) => match state.oracle {
                Some(oracle) => oracle,
                None => return Err(ContractError::OracleNotSet {}),
            },
            Err(_) => return Err(ContractError::ConfigNotFound {}),
        };

        // Fetch the latest price from the oracle
        let latest_price: Vec<Coin> = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: oracle_address.to_string(),
            msg: to_json_binary(&QueryMsg::LatestPrice {oracle: oracle_address.to_string()})?.into(), 
        }))?;

        // Update the option's price with the latest from the oracle
        option.price = latest_price.clone();

        ConstellationDerivativeEvent::emit_price_updated_with_oracle(deps.as_ref(), price)?;
        

        MARKET_LIST.save(deps.storage, id, &option)?;

        let res: Response = Response::new().add_attributes([("action", "update_price_oracle")]);
        Ok(res)
    }


    // Set conditions for option exercise, such as specific events or external data criteria.
    pub fn execute_set_option_exercise_conditions(
        deps: DepsMut,
        info: MessageInfo,
        id: u64,
        exercise_conditions: Vec<String>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let mut option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate the sender is the creator of the option
        if info.sender != option.creator {
            return Err(ContractError::Unauthorized {});
        }

        // Update the option exercise conditions
        option.exercise_conditions = Some(exercise_conditions.clone());

        ConstellationDerivativeEvent::emit_option_exercise_conditions_set(deps.as_ref(),  id, exercise_conditions)?;


        // Save the updated option
        OPTION_LIST.save(deps.storage, id, &option)?;

        // Return a success response
        Ok(Response::new().add_attribute("action", "set_option_exercise_conditions"))
    }

    // Allow the setting of additional option parameters like strike prices, exercise styles, and other contract-specific details.
    pub fn execute_set_option_parameters(
        deps: DepsMut,
        info: MessageInfo,
        id: u64,
        parameters: HashMap<String, String>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let mut option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate the sender is the creator of the option
        if info.sender != option.creator {
            return Err(ContractError::Unauthorized {});
        }

        // Update the option parameters
        option.parameters = Some(parameters.clone());

        ConstellationDerivativeEvent::emit_option_parameters_set(deps.as_ref(),  id, parameters)?;
        
        // Save the updated option
        OPTION_LIST.save(deps.storage, id, &option)?;

        // Return a success response
        Ok(Response::new().add_attribute("action", "set_option_parameters"))
    }

    // Notify option holders and the contract when an option is about to expire.
    pub fn execute_notify_option_expiry(
        deps: DepsMut,
        env: Env,
        id: u64,
        notification_period: u64,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate that the current time is within the notification period before expiry
        if option.expires.seconds() - env.block.time.seconds() <= notification_period {
            // Notify option holders
            // (This is a placeholder and should be replaced with the actual logic to notify option holders)

            ConstellationDerivativeEvent::emit_option_expired_notification(deps.as_ref(),  id, notification_period)?;
            // Construct a response with the notification information
            let res = Response::new().add_attribute(
                "action",
                "notify_option_expiry",
            );

            // Return the response
            Ok(res)
        } else {
            // If not within the notification period, return an error
            Err(ContractError::InvalidExpirationNotification {})
        }
    }


    // Get historical data of an option, such as previous prices, exercise events, and other relevant information.
    pub fn execute_get_option_history(deps: DepsMut, id: u64) -> Result<Vec<String>, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        ConstellationDerivativeEvent::emit_option_history(deps.as_ref(), id, Vec::<String>::new())?;

        // Return the historical data (This is a placeholder and should be replaced with actual historical data)
        Ok(option.history)
        
    }

    // Calculate risk metrics for an option, such as the delta, gamma, theta, and other parameters.
    pub fn execute_calculate_option_risk_metrics(deps: DepsMut, id: u64) -> Result<HashMap<String, Decimal>, ContractError> {
        // Load the option data
        let _option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Calculate risk metrics (This is a placeholder and should be replaced with actual calculations or info from optin above)
        let mut risk_metrics = HashMap::new();
        risk_metrics.insert("delta".to_string(), Decimal::zero());
        risk_metrics.insert("gamma".to_string(), Decimal::zero());
        risk_metrics.insert("theta".to_string(), Decimal::zero());
        // Add more risk metrics calculations as needed

        ConstellationDerivativeEvent::emit_option_risk_metrics_calculated(deps.as_ref(),  id, risk_metrics.clone())?;

        Ok(risk_metrics)
    }

    // Allow users to provide liquidity to the option pool, receiving LP tokens in return.
    pub fn execute_provide_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: u64,
        liquidity_amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate that the current time is before the option expiration
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Perform the liquidity provision logic (This is a placeholder and should be replaced with actual logic)
        // Assume LP tokens are minted and sent to the liquidity provider
        let mut res = Response::new();
        res = res.add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: liquidity_amount.clone(),
        });

        ConstellationDerivativeEvent::emit_liquidity_provided(deps.as_ref(), info.sender.to_string(), serde_json::to_string(&liquidity_amount).map_err(|_| ContractError::JsonSerializationError {})?)?;
        // Update the option state or pool balances as needed

        res = res.add_attribute("action", "provide_liquidity");
        Ok(res)
    }

    // Allow users to withdraw liquidity from the option pool, burning LP tokens.
    pub fn execute_withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: u64,
        liquidity_amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Load the option data
        let option = match OPTION_LIST.load(deps.storage, id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        // Validate that the current time is before the option expiration
        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Perform the liquidity withdrawal logic (This is a placeholder and should be replaced with actual logic)
        // Assume LP tokens are burned and the corresponding assets are sent to the liquidity provider
        let mut res = Response::new();
        res = res.add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: liquidity_amount.clone(),
        });
        // Update the option state or pool balances as needed

        ConstellationDerivativeEvent::emit_liquidity_withdrawn(deps.as_ref(), info.sender.to_string(), serde_json::to_string(&liquidity_amount).map_err(|_| ContractError::JsonSerializationError {})?)?;

        res = res.add_attribute("action", "withdraw_liquidity");
        Ok(res)
    }

    // Allow users to vote on a governance proposal related to the option protocol.
    pub fn execute_vote_on_governance_proposal(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo, //may be used when governance proposal is fully implemented
        proposal_id: u64,
        vote: bool,
    ) -> Result<Response, ContractError> {
        // Placeholder logic for governance voting
        // Assume there is a separate governance contract, and this contract communicates with it.
        // This should be replaced with the actual logic for interacting with the governance module.

        // Validate that the current time is before the option expiration
        let option_id = proposal_id; // Assuming the proposal ID corresponds to an option ID
        let option = match OPTION_LIST.load(deps.storage, option_id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), proposal_id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Placeholder logic for voting (replace with actual governance contract interaction)
        // For illustration purposes, assume we send a message to a governance contract to register the vote.
        // This will likely involve interacting with the governance contract's vote method.

        ConstellationDerivativeEvent::emit_vote_on_governance_proposal(deps.as_ref(), proposal_id, vote)?;
        let governance_contract = Addr::unchecked("your_governance_contract_address"); // Replace with actual address
        let governance_msg = GovernanceMsg::Vote {
            proposal_id,
            vote,
        };
        let mut res = Response::new();
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: governance_contract.into(),
            msg: to_json_binary(&governance_msg)?,
            funds: vec![],
        }));

        res = res.add_attribute("action", "vote_on_governance_proposal");
        Ok(res)
    }

    // Allow users to use an option as collateral for other financial activities.
    pub fn execute_use_option_as_collateral(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo, //may be used during full implementation fo placeholder logic
        option_id: u64,
        amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Placeholder logic for using option as collateral
        // This should be replaced with the actual logic for interacting with other financial modules.

        // Validate that the current time is before the option expiration
        let option = match OPTION_LIST.load(deps.storage, option_id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Placeholder logic for using option as collateral (replace with actual logic)
        // This could involve sending a message to another module that supports using options as collateral.
        // For illustration purposes, assume we send a message to a hypothetical collateral module.

        ConstellationDerivativeEvent::emit_option_used_as_collateral(deps.as_ref(), option_id, amount.clone())?;

        let collateral_module = Addr::unchecked("your_collateral_module_address"); // Replace with actual address
        let collateral_msg = CollateralMsg::UseOptionAsCollateral {
            option_id,
            amount,
        };
        let mut res = Response::new();
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collateral_module.into(),
            msg: to_json_binary(&collateral_msg)?,
            funds: vec![],
        }));

        res = res.add_attribute("action", "use_option_as_collateral");
        Ok(res)
    }

    // Wrap an option to participate in a yield farming program.
    pub fn execute_wrap_option_for_yield_farming(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,  //may be used during full implementation fo placeholder logic
        option_id: u64,
        amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        // Placeholder logic for wrapping option for yield farming
        // This should be replaced with the actual logic for interacting with a yield farming module.

        // Validate that the current time is before the option expiration
        let option = match OPTION_LIST.load(deps.storage, option_id) {
            Ok(option) => option,
            Err(error) => return Err(ContractError::Std(error)),
        };

        if env.block.time >= option.expires {
            ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
            return Err(ContractError::OptionExpired { expired: option.expires });
        }

        // Placeholder logic for wrapping option for yield farming (replace with actual logic)
        // This could involve sending a message to another module that supports wrapping options for yield farming.
        // For illustration purposes, assume we send a message
        ConstellationDerivativeEvent::emit_option_wrapped_for_yield_farming(deps.as_ref(), option_id, amount.clone())?;
        
        let yield_farming_module = Addr::unchecked("your_yield_farming_module_address"); // Replace with actual address
        let
        
        
        yield_farming_msg = YieldFarmingMsg::WrapOptionForYieldFarming {
            option_id,
            amount,
        };
        let mut res = Response::new();
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: yield_farming_module.into(),
            msg: to_json_binary(&yield_farming_msg)?,
            funds: vec![],
        }));

        res = res.add_attribute("action", "wrap_option_for_yield_farming");
        Ok(res)
    }



// Create an Automated Market Maker (AMM) pool for trading options.
pub fn execute_create_amm_pool(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo, //might use during full implementation
    option_id: u64,
) -> Result<Response, ContractError> {
    // Placeholder logic for creating AMM pool
    // This should be replaced with the actual logic for creating an AMM pool.

    // Validate that the current time is before the option expiration
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    ConstellationDerivativeEvent::emit_amm_pool_created(deps.as_ref(), option_id)?;

    // Placeholder logic for creating AMM pool (replace with actual logic)
    // This could involve sending a message to another module that supports AMM pool creation.
    // For illustration purposes, assume we send a message to a hypothetical AMM module.
    let amm_module = Addr::unchecked("your_amm_module_address"); // Replace with actual address
    let amm_msg = AmmMsg::CreatePool { option_id };
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: amm_module.into(),
        msg: to_json_binary(&amm_msg)?,
        funds: vec![],
    }));

    res.clone().add_attribute("action", "create_amm_pool");
    Ok(res)
}

// Trade options on an existing Automated Market Maker (AMM) pool.
pub fn execute_trade_on_amm(
    deps: DepsMut,
    _env: Env, //might use during full implementation
    _info: MessageInfo, //might use during full implementation
    pool_id: u64,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    // Placeholder logic for trading on AMM
    // This should be replaced with the actual logic for trading options on an AMM pool.

    // Placeholder logic for validating trade conditions (replace with actual validation logic)
    if amount.is_empty() {
        return Err(ContractError::InvalidTradeAmount {});
    }

    ConstellationDerivativeEvent::emit_option_traded_on_amm(deps.as_ref(), pool_id, amount.clone())?;

    // Placeholder logic for trading on AMM (replace with actual logic)
    // This could involve sending a message to another module that supports AMM trading.
    // For illustration purposes, assume we send a message to a hypothetical AMM module.
    let amm_module = Addr::unchecked("your_amm_module_address"); // Replace with actual address
    let amm_msg = AmmMsg::Trade { pool_id, amount };
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: amm_module.into(),
        msg: to_json_binary(&amm_msg)?,
        funds: vec![],
    }));

    res.clone().add_attribute("action", "trade_on_amm");
    Ok(res)
}

// Integrate market data feed for options pricing.
pub fn execute_integrate_market_data_feed(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo, //might use during full implementation
    option_id: u64,
    data_feed_url: String,
) -> Result<Response, ContractError> {
    // Placeholder logic for integrating market data feed
    // This should be replaced with the actual logic for integrating a market data feed.

    // Validate that the current time is before the option expiration
    let option = match OPTION_LIST.load(deps.storage, option_id) {
        Ok(option) => option,
        Err(error) => return Err(ContractError::Std(error)),
    };

    if env.block.time >= option.expires {
        ConstellationDerivativeEvent::emit_option_expired(deps.as_ref(), option_id)?;
        return Err(ContractError::OptionExpired { expired: option.expires });
    }

    ConstellationDerivativeEvent::emit_market_data_feed_integrated(deps.as_ref(), option_id, data_feed_url.clone())?;

    // Placeholder logic for integrating market data feed (replace with actual logic)
    // This could involve sending a message to another module that supports data feed integration.
    // For illustration purposes, assume we send a message to a hypothetical data feed module.
    let data_feed_module = Addr::unchecked("your_data_feed_module_address"); // Replace with actual address
    let data_feed_msg = DataFeedMsg::Integrate { option_id, data_feed_url };
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: data_feed_module.into(),
        msg: to_json_binary(&data_feed_msg)?,
        funds: vec![],
    }));

    res.clone().add_attribute("action", "integrate_market_data_feed");
    Ok(res)
}

pub fn execute_refer_user(
    deps: DepsMut,
    info: MessageInfo,
    referred_user: Addr,
) -> Result<Response, ContractError> {
    // Placeholder logic for referral program
    // This should be replaced with the actual logic for tracking referrals and providing rewards.

    // Assume a referral reward structure
    let referral_reward = Coin {
        denom: "uusd".to_string(),
        amount: Uint128::new(100), // Adjust the reward amount as needed
    };

    ConstellationDerivativeEvent::emit_user_referred(deps.as_ref(), referred_user.clone())?;

    // Transfer referral reward to the referring user
    let res = Response::new();
    res.clone().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![referral_reward],
    }));

    // Add attributes for referral
    res.clone().add_attribute("action", "refer_user");
    res.clone().add_attribute("referring_user", info.sender);
    res.clone().add_attribute("referred_user", referred_user);

    Ok(res)
}


pub fn execute_set_discount_criteria(
    deps: DepsMut,
    info: MessageInfo,
    criteria: HashMap<String, String>,
) -> Result<Response, ContractError> {
    // Placeholder logic for setting discount criteria
    // This should be replaced with the actual logic for allowing users to set their own discount criteria.

    // Assume a discount criteria structure in the contract state

    let sender_raw = deps.api.addr_validate(&(&info.sender).to_string())?;
    let mut discounts = DISCOUNTS.load(deps.storage, &sender_raw)?;
    //discounts.insert
    discounts.criteria.insert(info.sender.clone().to_string(), criteria.clone());
    DISCOUNTS.save(deps.storage, &sender_raw, &discounts)?;

    ConstellationDerivativeEvent::emit_discount_criteria_set(deps.as_ref(), info.sender.clone(), criteria)?;

    // Add attributes for setting discount criteria
    let res = Response::new();
    res.clone().add_attribute("action", "set_discount_criteria");
    res.clone().add_attribute("user", info.sender);

    Ok(res)
}

/*
pub fn execute_create_token(
    deps: DepsMut, info: MessageInfo, counter_offer: Coin,time_stamp: Timestamp,) -> Result<Response, ContractError> {
    // Validate the sender
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Create a new option with token as collateral
    let new_option = Option {
        id: generate_option_id(deps.api, &info.sender, &time_stamp),
        creator: info.sender.clone(),
        owner: info.sender.clone(),
        collateral: counter_offer,
        counter_offer: Coin {
            denom: "uatom".to_string(),
            amount: "0".to_string(),
        },
        expires: time_stamp + config.option_duration,
        status: OptionStatus::Active,
        price: Vec::new(),
        onsale: false,
    };

    ConstellationDerivativeEvent::emit_token_option_created(deps.as_ref(), new_option.id.clone(), info.sender.clone())?;

    // Save the new option to storage
    OPTION_LIST.save(deps.storage, new_option.id.clone(), &new_option)?;

    let res: Response = Response::new().add_attributes([("action", "create_token")]);
    Ok(res)
}

// Sample Function to generate an option ID if needed
fn generate_option_id(api: &dyn Api, creator: &str, time_stamp: &u64) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(creator.as_bytes());
    bytes.extend_from_slice(&time_stamp.to_be_bytes());

    // You can customize this part based on your requirements
    let hash = api.sha256(&bytes);
    hex::encode(hash)
}
pub fn execute_lock_tokens(
    deps: DepsMut, env: Env, info: MessageInfo, amount: Vec<Coin>, lock_duration: u64,) -> Result<Response, ContractError> {
    // Placeholder logic for locking tokens
    // This should be replaced with the actual logic for locking tokens and providing exclusive access.
    // Assume a lock structure and add the user to the lock list
    let lock = Lock {
        expiration: env.block.time.seconds() + lock_duration, 
        user: info.sender.clone(),
        locked_amount: amount.clone(),
    };
    LOCKS.save(deps.storage, &info.sender, &lock)?;

    // Assume an ERC-20 token contract and transfer tokens to this contract
    let token_contract = Addr::unchecked("your_token_contract_address"); // Replace with actual address
    let lock_msg = Cw20ExecuteMsg::Transfer {
        recipient: env.contract.address.clone().into(),
        amount,
    };

    ConstellationDerivativeEvent::emit_tokens_locked(deps.as_ref(), amount, lock_duration)?;

    let mut res = Response::new();
    res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_contract.into(),
        msg: to_json_binary(&lock_msg)?,
        funds: vec![],
    }));

    // Add attributes for locking tokens
    res.add_attribute("action", "lock_tokens");
    res.add_attribute("user", info.sender);

    Ok(res)
}
//sample liquidity implementtion 
pub struct Liquidity {
  pub locked: Uint128,
  pub unlocked: Uint128,
}
pub fn total_liquidity(deps: Deps) -> StdResult<Liquidity> {
  let locked = LOCKED_COINS
    .range(deps.storage, None, None, Order::Ascending])
    .map(|item| item.amount)
    .sum();
  let unlocked = UNLOCKED_COINS
    .range(deps.storage, None, None, Order::Ascending])  
    .map(|item| item.amount)
    .sum();
  Ok(Liquidity{
    locked,
    unlocked    
  })  
}
//IMPLEMENTATION OF ONLY OWNER MODIFIER

// Modify trait definition with two generic arguments
pub trait Modify<T> {
    fn modify(&self, deps: DepsMut, env: Env, msg: T) -> Result<T, ContractError>;
}
// Implement Modify for OnlyOwner
impl<T> Modify<T> for OnlyOwner {
    fn modify(&self, deps: DepsMut, env: Env, msg: T) -> Result<T, ContractError> {
        let sender = env.message.sender;
        let config: Config = CONFIG.load(deps.storage)?;
        if sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(msg)
    }
}

EXAMPLE USE OF onlyOwner Modifier
pub fn set_discount_criteria(
    deps: DepsMut, env: Env, info: MessageInfo, criteria: HashMap<String, String>,) -> Result<Response, ContractError> {
    let only_owner = OnlyOwner {};
    only_owner.modify(deps.branch(), env.clone(), ())?;    // this Ensures only owner can modify
    // Rest of your existing logic for setting discount criteria...
    Ok(res)
}
*/