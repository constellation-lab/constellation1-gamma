use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult, StdError, Timestamp, Uint128};
use cw20_base::contract::execute as cw20_execute;
use cw20_base::contract::query as cw20_query;
use cw20_base::msg::{ExecuteMsg as Cw20ExecuteMsg, QueryMsg as Cw20QueryMsg, InstantiateMsg};
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{StakeInfo, STAKE_INFO, PROPOSALS};
use crate::error::ContractError;
use crate::events::CDTTokenEvent;
use cw20::Cw20Coin;
use cw20_base::allowances::query_allowance;
use cw20_base::enumerable::{query_all_allowances, query_all_accounts};

pub struct Contract;

impl Contract {
    fn initialize(&self, deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<(), ContractError> {
        let initial_balances = vec![Cw20Coin {
            address: info.sender.to_string(),
            amount: Uint128::from(INITIAL_SUPPLY),
        }];

        cw20_base::contract::instantiate(
            deps,
            env,
            info,
            InstantiateMsg {
                name: "Constella Decision Token".to_string(),
                symbol: "CDT".to_string(),
                decimals: 6,
                initial_balances,
                mint: None,
                marketing: None,
            },
        ).map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))?;

        Ok(())
    }
}

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Contract::initialize(&Contract, deps, env, info, msg)?;
    Ok(Response::default())
}

pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            let res = cw20_execute(deps.branch(), env, info.clone(), Cw20ExecuteMsg::Transfer { recipient: recipient.clone(), amount })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))?;
            let event = CDTTokenEvent::Transfer {
                from: info.sender.clone(),
                to: deps.api.addr_validate(&recipient)?,
                amount,
            }.into_event()?;
            Ok(res.add_event(event))
        }
        ExecuteMsg::Burn { amount } => {
            let res = cw20_execute(deps.branch(), env, info.clone(), Cw20ExecuteMsg::Burn { amount })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))?;
            let event = CDTTokenEvent::Burn {
                owner: info.sender.clone(),
                amount,
            }.into_event()?;
            Ok(res.add_event(event))
        }
        ExecuteMsg::Mint { recipient, amount } => {
            let res = cw20_execute(deps.branch(), env, info.clone(), Cw20ExecuteMsg::Mint { recipient: recipient.clone(), amount })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))?;
            let event = CDTTokenEvent::Mint {
                recipient: deps.api.addr_validate(&recipient)?,
                amount,
            }.into_event()?;
            Ok(res.add_event(event))
        }
        ExecuteMsg::Send { contract, amount, msg } => {
            cw20_execute(deps.branch(), env, info, Cw20ExecuteMsg::Send { contract, amount, msg })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))
        }
        ExecuteMsg::IncreaseAllowance { spender, amount } => {
            cw20_execute(deps.branch(), env, info, Cw20ExecuteMsg::IncreaseAllowance { spender, amount, expires: None })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))
        }
        ExecuteMsg::DecreaseAllowance { spender, amount } => {
            cw20_execute(deps.branch(), env, info, Cw20ExecuteMsg::DecreaseAllowance { spender, amount, expires: None })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))
        }
        ExecuteMsg::TransferFrom { owner, recipient, amount } => {
            cw20_execute(deps.branch(), env, info, Cw20ExecuteMsg::TransferFrom { owner, recipient, amount })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))
        }
        ExecuteMsg::BurnFrom { owner, amount } => {
            cw20_execute(deps.branch(), env, info, Cw20ExecuteMsg::BurnFrom { owner, amount })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))
        }
        ExecuteMsg::SendFrom { owner, contract, amount, msg } => {
            cw20_execute(deps.branch(), env, info, Cw20ExecuteMsg::SendFrom { owner, contract, amount, msg })
                .map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))
        }
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps.branch(), env, info, proposal_id, vote),
    }
}

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
    vote: bool,
) -> Result<Response, ContractError> {
      // Check if the proposal exists
      let proposal = PROPOSALS.may_load(deps.storage, proposal_id)?;
      if proposal.is_none() {
          return Err(ContractError::ProposalNotFound {});
      }
      let mut proposal = proposal.unwrap();
  
    // Check if the voting period has ended
    if env.block.time > Timestamp::from_seconds(proposal.voting_end_time) {
        return Err(ContractError::VotingPeriodEnded {});
    }



    // Get the voter's CDT balance as their voting power
    let voter_address = info.sender;
    //let voting_power = cw20_query::balance(deps.as_ref(), voter_address.clone())?;
    //let voting_power = cw20_base::contract::query_balance(deps.as_ref(), voter_address.clone())?.balance;
    let voting_power = cw20_base::contract::query_balance(deps.as_ref(), voter_address.clone().to_string())?.balance;
   

    // Check if the voter has already voted
    if proposal.votes.iter().any(|(addr, _)| addr == &voter_address) {
        return Err(ContractError::AlreadyVoted {});
    }

    // Record the vote
    proposal.votes.push((voter_address.clone(), vote));


    // Update the total voting power
    if vote {
        proposal.total_yes_votes += voting_power;
    } else {
        proposal.total_no_votes += voting_power;
    }

    // Save the updated proposal
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    // Emit the vote event
    let event = Event::new("vote")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("voter", voter_address.to_string())
        .add_attribute("vote", vote.to_string())
        .add_attribute("voting_power", voting_power.to_string());

    Ok(Response::new().add_event(event))
}


fn execute_stake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone();

    // Update the staking info for the sender
    let mut stake_info = STAKE_INFO.may_load(deps.storage, sender.clone())?.unwrap_or(StakeInfo { staked_amount: Uint128::zero() });
    stake_info.staked_amount += amount;
    STAKE_INFO.save(deps.storage, sender.clone(), &stake_info)?;

    Ok(Response::new()
        .add_attribute("action", "stake")
        .add_attribute("staker", sender)
        .add_attribute("amount", amount.to_string()))
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => {
            let balance = cw20_base::contract::query_balance(deps, address)?;
            to_binary(&balance)
        }
        QueryMsg::TokenInfo {} => {
            let token_info = cw20_base::contract::query_token_info(deps)?;
            to_binary(&token_info)
        }
        QueryMsg::Allowance { owner, spender } => {
            let allowance = query_allowance(deps, owner, spender)?;
            to_binary(&allowance)
        }
        QueryMsg::AllAllowances { owner, start_after, limit } => {
            let allowances = query_all_allowances(deps, owner, start_after, limit)?;
            to_binary(&allowances)
        }
        QueryMsg::AllAccounts { start_after, limit } => {
            let accounts = query_all_accounts(deps, start_after, limit)?;
            to_binary(&accounts)
        }
        QueryMsg::Minter {} => {
            let minter = cw20_base::contract::query_minter(deps)?;
            to_binary(&minter)
        }
        QueryMsg::MarketingInfo {} => {
            let marketing_info = cw20_base::contract::query_marketing_info(deps)?;
            to_binary(&marketing_info)
        }
        QueryMsg::DownloadLogo {} => {
            let logo = cw20_base::contract::query_download_logo(deps)?;
            to_binary(&logo)
        }
        QueryMsg::VotingPower { address } => {
            let voting_power = cw20_base::contract::query_balance(deps, address)?.balance;
            to_binary(&voting_power)
        }
    }
        
    
}

const INITIAL_SUPPLY: u128 = 1_000_000_000; // Set the initial supply of CDT tokens






//By removing the #[entry_point] attributes in instantiate, execute and query, the contract will rely on the cw20_base library to handle the entry points, and your custom logic will be executed when the corresponding messages are received.

/* older useful sample related information
The #[entry_point] attribute has been removed from the execute and query functions.
The instantiate import and function have been removed since they are not used in this code snippet.

Now, the cw20_base library will handle the entry points for execution and querying, and your execute and query functions will be called internally by the library.
With these changes, the duplicate symbol errors should be resolved.

The cw20_base library provides a set of standard entry points and implementations for the CW20 token standard. When you use the cw20_base library in your contract, it takes care of handling the common functionality and entry points, such as instantiate, execute, and query.
Here's how it works:

When someone interacts with your contract, they will send messages to the standard CW20 entry points, such as instantiate, execute, or query.
The cw20_base library receives these messages and routes them to the appropriate functions based on the message type.
If the message is a standard CW20 message (e.g., Transfer, Burn, Send), the cw20_base library handles it internally using its own implementation.
If the message is a custom message that you have defined in your contract (e.g., Stake), the cw20_base library calls your corresponding function (e.g., execute_stake) to handle the custom logic.

old sample info:
Here's an example of how someone would interact with your contract:

use cosmwasm_std::{to_binary, Addr, Coin, Uint128};
use cw20::msg::ExecuteMsg;

// Instantiate the contract
let init_msg = cw20_base::msg::InstantiateMsg {
    name: "CPST Token".to_string(),
    symbol: "CPST".to_string(),
    decimals: 6,
    initial_balances: vec![],
    mint: None,
};
let info = MessageInfo {
    sender: Addr::unchecked("owner"),
    funds: vec![],
};
let res = instantiate(deps, env, info, init_msg);

// Execute a transfer
let transfer_msg = ExecuteMsg::Transfer {
    recipient: Addr::unchecked("recipient"),
    amount: Uint128::new(1000),
};
let info = MessageInfo {
    sender: Addr::unchecked("sender"),
    funds: vec![],
};
let res = execute(deps, env, info, transfer_msg);

// Execute a custom staking message
let stake_msg = ExecuteMsg::Send {
    contract: env.contract.address.to_string(),
    amount: Uint128::new(500),
    msg: to_binary(&cw20_stake::msg::ReceiveMsg::Stake {}).unwrap(),
};
let info = MessageInfo {
    sender: Addr::unchecked("staker"),
    funds: vec![],
};
let res = execute(deps, env, info, stake_msg);
*/