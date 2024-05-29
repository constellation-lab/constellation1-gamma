use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, Addr, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, Order,
};
use cw20::{Cw20ReceiveMsg, Cw20ExecuteMsg};
use cw20_base::contract::{execute as cw20_execute, query as cw20_query};
use cw20_base::state::{TOKEN_INFO, BALANCES};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PoolInfo, CnsteStakingProgramInfo };
use crate::state::{Config, YieldFarmingProgram, LiquidityMiningProgram, CONFIG, YIELD_FARMING_PROGRAMS, LIQUIDITY_MINING_PROGRAMS, USER_STAKED_AMOUNT, USER_REWARD_PER_TOKEN_PAID, LIQUIDITY_PROVIDERS, TOTAL_FEES_DISTRIBUTED};
use crate::state::{CNSTE_STAKING_PROGRAMS, USER_CNSTE_STAKED_AMOUNT, TOTAL_CNSTE_STAKED_AMOUNT, CNSTE_REWARD_POOL, CnsteStakingProgram };



const LIQUIDITY_POOL_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const CONSTELLA_OPTION_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const PRICING_ORACLE_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const OPTION_MARKETPLACE_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";

const CDT_TOKEN_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const CPST_TOKEN_CONTRACT: &str = "nibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";
const CNSTE_TOKEN_CONTRACT: &str = "cnibi1np4pvfw5fe9nj6ds3fv8v9cm9d8umasm8ew8p2r269anqnchws9q5eq7l9";


pub struct Contract;

impl Contract {
    fn initialize(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: cw20_base::msg::InstantiateMsg,
        custom_msg: &crate::msg::InstantiateMsg,
    ) -> Result<(), ContractError> {
        cw20_base::contract::instantiate(
            deps.branch(),
            env,
            info,
            msg,
        ).map_err(|err| ContractError::Std(StdError::generic_err(format!("{}", err))))?;

        // Initialize the custom fields from your InstantiateMsg struct
        let config = Config {
            governance_token: custom_msg.governance_token.clone(),
            liquidity_pool_contract: deps.api.addr_validate(&custom_msg.liquidity_pool_contract)?,
            constella_option_contract: deps.api.addr_validate(&custom_msg.constella_option_contract)?,
            pricing_oracle_contract: deps.api.addr_validate(&custom_msg.pricing_oracle_contract)?,
            option_marketplace_contract: deps.api.addr_validate(&custom_msg.option_marketplace_contract)?,

            cdt_token_contract: deps.api.addr_validate(&custom_msg.cdt_token_contract)?,
            cpst_token_contract: deps.api.addr_validate(&custom_msg.cpst_token_contract)?,
            cnste_token_contract: deps.api.addr_validate(&custom_msg.cnste_token_contract)?,
            fee_distributor: deps.api.addr_validate(&custom_msg.fee_distributor)?,
        };

        CONFIG.save(deps.storage, &config)?;

        Ok(())
    }
}

//#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: cw20_base::msg::InstantiateMsg,
    custom_msg: crate::msg::InstantiateMsg,
) -> Result<Response, ContractError> {
    Contract::initialize(&Contract, deps.branch(), env, info, msg, &custom_msg)?;
    Ok(Response::default())
}



//#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateYieldFarmingProgram {
            program_id,
            reward_token,
            reward_rate,
            start_time,
            end_time,
        } => execute_create_yield_farming_program(
            deps,
            env,
            info,
            program_id,
            reward_token,
            reward_rate,
            start_time,
            end_time,
        ),
        ExecuteMsg::CreateLiquidityMiningProgram {
            program_id,
            option_pair,
            reward_multiplier,
            start_time,
            end_time,
        } => execute_create_liquidity_mining_program(
            deps,
            env,
            info,
            program_id,
            option_pair,
            reward_multiplier,
            start_time,
            end_time,
        ),
        ExecuteMsg::CreateCnsteStakingProgram {
            program_id,
            reward_per_token_stored,
            last_update_time,
        } => execute_create_cnste_staking_program(
            deps,
            env,
            info,
            program_id,
            reward_per_token_stored,
            last_update_time,
        ),
        ExecuteMsg::Stake { program_id, amount } => execute_stake(deps, env, info, program_id, amount),
        ExecuteMsg::Unstake { program_id, amount } => execute_unstake(deps, env, info, program_id, amount),
        ExecuteMsg::ClaimRewards { program_id } => execute_claim_rewards(deps, env, info, program_id),
        ExecuteMsg::DistributePerformanceFees { amount } => execute_distribute_performance_fees(deps, env, info, amount),
        ExecuteMsg::Receive(msg) => execute_receive(deps, env, info, msg),
 
    }
}


    
fn execute_create_yield_farming_program(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: String,
    reward_token: String,
    reward_rate: Uint128,
    start_time: u64,
    end_time: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
  
    // Only the governance token holder can create yield farming programs
    if info.sender != config.governance_token {
        return Err(ContractError::Unauthorized {});
    }

    let program = YieldFarmingProgram {
        program_id,
        reward_token,
        reward_rate,
        start_time,
        end_time,
        total_staked: Uint128::zero(),
        reward_per_token_stored: Uint128::zero(),
        last_update_time: env.block.time.seconds(),
    };

    YIELD_FARMING_PROGRAMS.save(deps.storage, program.program_id.clone(), &program)?;

    Ok(Response::default())
}

fn execute_create_liquidity_mining_program(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: String,
    option_pair: String,
    reward_multiplier: Uint128,
    start_time: u64,
    end_time: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only the governance token holder can create liquidity mining programs
    if info.sender != config.governance_token {
        return Err(ContractError::Unauthorized {});
    }

    let program = LiquidityMiningProgram {
        program_id,
        option_pair,
        reward_multiplier,
        start_time,
        end_time,
        total_liquidity: Uint128::zero(),
        reward_per_token_stored: Uint128::zero(),
        last_update_time: env.block.time.seconds(),
    };

    LIQUIDITY_MINING_PROGRAMS.save(deps.storage, program.program_id.clone(), &program)?;

    Ok(Response::default())
}

fn execute_create_cnste_staking_program(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: String,
    reward_per_token_stored: Uint128,
    last_update_time: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only the governance token holder can create liquidity mining programs
    if info.sender != config.governance_token {
        return Err(ContractError::Unauthorized {});
    }

    let program = CnsteStakingProgram {
        program_id,
        reward_per_token_stored: Uint128::zero(),
        last_update_time: env.block.time.seconds(),
    };

    CNSTE_STAKING_PROGRAMS.save(deps.storage, program.program_id.clone(), &program)?;

    Ok(Response::default())
}

fn execute_stake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Check if the program exists
    if let Some(mut program) = YIELD_FARMING_PROGRAMS.may_load(deps.storage, program_id.clone())? {
        // Update the user's staked amount
        let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (info.sender.clone(), program_id.clone())).unwrap_or_default();
        USER_STAKED_AMOUNT.save(deps.storage, (info.sender.clone(), program_id.clone()), &(user_staked_amount + amount))?;

        // Update the total staked amount
        program.total_staked += amount;
        YIELD_FARMING_PROGRAMS.save(deps.storage, program_id.clone(), &program)?;

        // Calculate and update the reward per token
        let current_time = env.block.time.seconds();
        let time_elapsed = current_time - program.last_update_time;
        let reward = program.reward_rate * Uint128::from(time_elapsed);
        let reward_per_token = if program.total_staked == Uint128::zero() {
            Uint128::zero()
        } else {
            reward.multiply_ratio(Uint128::from(1u128), program.total_staked)
        };
        program.reward_per_token_stored += reward_per_token;
        program.last_update_time = current_time;
        YIELD_FARMING_PROGRAMS.save(deps.storage, program_id.clone(), &program)?;
    } else if let Some(mut program) = LIQUIDITY_MINING_PROGRAMS.may_load(deps.storage, program_id.clone())? {
        // Update the user's staked amount
        let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (info.sender.clone(), program_id.clone()))?;
        USER_STAKED_AMOUNT.save(deps.storage, (info.sender.clone(), program_id.clone()), &(user_staked_amount - amount))?;

        // Update the total liquidity
        program.total_liquidity += amount;
        LIQUIDITY_MINING_PROGRAMS.save(deps.storage, program_id.clone(), &program)?;

        // Calculate and update the reward per token
        let current_time = env.block.time.seconds();
        let time_elapsed = current_time - program.last_update_time;
        let reward = program.reward_multiplier * Uint128::from(time_elapsed);
        let reward_per_token = if program.total_liquidity == Uint128::zero() {
            Uint128::zero()
        } else {
            reward.multiply_ratio(Uint128::from(1u128), program.total_liquidity)
        };
        program.reward_per_token_stored += reward_per_token;
        program.last_update_time = current_time;
        LIQUIDITY_MINING_PROGRAMS.save(deps.storage, program_id.clone(), &program)?;


    } else if program_id == "cnste_staking" {
        // Update the user's CNSTE staked amount
        let user_cnste_staked_amount = USER_CNSTE_STAKED_AMOUNT
            .load(deps.storage, (info.sender.clone(), program_id.clone()))
            .unwrap_or_default();
        USER_CNSTE_STAKED_AMOUNT.save(
            deps.storage,
            (info.sender.clone(), program_id.clone()),
            &(user_cnste_staked_amount + amount),
        )?;

        // Update the total CNSTE staked amount
        let total_cnste_staked_amount = TOTAL_CNSTE_STAKED_AMOUNT
            .load(deps.storage, program_id.clone())
            .unwrap_or_default();
        TOTAL_CNSTE_STAKED_AMOUNT.save(
            deps.storage,
            program_id.clone(),
            &(total_cnste_staked_amount + amount),
        )?;

        // Transfer CNSTE tokens from the user to the contract
        let cnste_token_contract = deps.api.addr_validate(&config.cnste_token_contract.to_string())?;
        let cw20_msg = Cw20ExecuteMsg::TransferFrom {
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount,
        };
        let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), cw20_msg)?;

        return Ok(Response::new()
            .add_submessages(cw20_res.messages)
            .add_attribute("action", "stake_cnste")
            .add_attribute("program_id", program_id)
            .add_attribute("user", info.sender.to_string())
            .add_attribute("amount", amount.to_string()));    
    } else {
        return Err(ContractError::ProgramNotFound {});
    }

    // Transfer the staked tokens from the user to the contract
    /*let cw20_msg = Cw20ReceiveMsg {
        sender: info.sender.to_string(),
        amount,
        msg: to_binary(&ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: info.sender.to_string(),
            amount,
            msg: to_binary(&ExecuteMsg::Stake {
                program_id: program_id.clone(),
                amount,
            })?,
        }))?,
    };*/

    let cw20_contract = deps.api.addr_validate(&config.liquidity_pool_contract.to_string())?;
    //let cw20_res = cw20_execute(deps.branch(), env, cw20_msg, vec![], cw20_contract)?;
    //let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), cw20_msg)?;
    let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), Cw20ExecuteMsg::Send {
        contract: cw20_contract.to_string(),
        amount,
        msg: to_binary(&ExecuteMsg::Stake {
            program_id: program_id.clone(),
            amount,
        })?,
    })?;

    Ok(Response::new()
        .add_submessages(cw20_res.messages)
        .add_attribute("action", "stake")
        .add_attribute("program_id", program_id)
        .add_attribute("user", info.sender.to_string())
        .add_attribute("amount", amount.to_string()))
}

fn execute_unstake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Check if the program exists
    if let Some(mut program) = YIELD_FARMING_PROGRAMS.may_load(deps.storage, program_id.clone())? {
        // Check if the user has sufficient staked amount
        //let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (&info.sender, &program_id))?;
        let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (info.sender.clone(), program_id.clone())).unwrap_or_default();
        if user_staked_amount < amount {
            return Err(ContractError::InsufficientStakedAmount {});
        }

        USER_STAKED_AMOUNT.save(deps.storage, (info.sender.clone(), program_id.clone()), &(user_staked_amount - amount))?;

        // Update the total staked amount
        program.total_staked -= amount;
        YIELD_FARMING_PROGRAMS.save(deps.storage, program_id.clone(), &program)?;
    } else if let Some(mut program) = LIQUIDITY_MINING_PROGRAMS.may_load(deps.storage, program_id.clone())? {
        // Check if the user has sufficient staked amount
        //let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (&info.sender, &program_id))?;
        let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (info.sender.clone(), program_id.clone())).unwrap_or_default();
        
        if user_staked_amount < amount {
            return Err(ContractError::InsufficientStakedAmount {});
        }

        // Update the user's staked amount
        //USER_STAKED_AMOUNT.save(deps.storage, (&info.sender, &program_id), &(user_staked_amount - amount))?;
        USER_STAKED_AMOUNT.save(deps.storage, (info.sender.clone(), program_id.clone()), &(user_staked_amount + amount))?;

        // Update the total liquidity
        program.total_liquidity -= amount;
        LIQUIDITY_MINING_PROGRAMS.save(deps.storage, program_id.clone(), &program)?;
    } else {
        return Err(ContractError::ProgramNotFound {});
    }

    // Transfer the unstaked tokens from the contract to the user
    let cw20_contract = deps.api.addr_validate(&config.liquidity_pool_contract.to_string())?;
    let cw20_msg = Cw20ExecuteMsg::Transfer {
        recipient: info.sender.to_string(),
        amount,
    };
    //let cw20_res = cw20_execute(deps.branch(), env, cw20_msg, vec![], cw20_contract)?;
    //let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), cw20_msg)?;
    let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), Cw20ExecuteMsg::Transfer {
        recipient: info.sender.to_string(),
        amount,
    })?;

    Ok(Response::new()
        .add_submessages(cw20_res.messages)
        .add_attribute("action", "unstake")
        .add_attribute("program_id", program_id)
        .add_attribute("user", info.sender.to_string())
        .add_attribute("amount", amount.to_string()))
}


fn execute_claim_rewards(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Check if the program exists
    if let Some(program) = YIELD_FARMING_PROGRAMS.may_load(deps.storage, program_id.clone())? {
        // Calculate the user's earned rewards
        let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (info.sender.clone(), program_id.clone()))?;
        let user_reward_per_token_paid = USER_REWARD_PER_TOKEN_PAID.load(deps.storage, (info.sender.clone(), program_id.clone())).unwrap_or_default();
        let reward_per_token = program.reward_per_token_stored;
        let pending_reward = user_staked_amount * (reward_per_token - user_reward_per_token_paid);

        // Update the user's reward per token paid
        USER_REWARD_PER_TOKEN_PAID.save(deps.storage, (info.sender.clone(), program_id.clone()), &reward_per_token)?;

        // Transfer the rewards to the user based on the reward token
        //let reward_token_contract = deps.api.addr_validate(&config.cnste_token_contract.to_string())?;

        let cw20_msg = Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: pending_reward,
        };
        //let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), cw20_msg)?;
        
        // For yield farming programs
        let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: pending_reward,
        })?;

        Ok(Response::new()
            .add_submessages(cw20_res.messages)
            .add_attribute("action", "claim_rewards")
            .add_attribute("program_id", program_id)
            .add_attribute("user", info.sender.to_string())
            .add_attribute("amount", pending_reward.to_string()))
    } else if let Some(program) = LIQUIDITY_MINING_PROGRAMS.may_load(deps.storage, program_id.clone())? {
 
        // Calculate the user's earned rewards
        let user_staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (info.sender.clone(), program_id.clone()))?;
        let user_reward_per_token_paid = USER_REWARD_PER_TOKEN_PAID.load(deps.storage, (info.sender.clone(), program_id.clone())).unwrap_or_default();
        let reward_per_token = program.reward_per_token_stored;
        let pending_reward = user_staked_amount * (reward_per_token - user_reward_per_token_paid);

        // Update the user's reward per token paid
        USER_REWARD_PER_TOKEN_PAID.save(deps.storage, (info.sender.clone(), program_id.clone()), &reward_per_token)?;

        // Transfer the rewards to the user for liquidity mining programs
        let reward_token_contract = deps.api.addr_validate(&config.cnste_token_contract.to_string())?;
        let cw20_msg = Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: pending_reward,
        };
        //let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), cw20_msg)?;
        // For liquidity mining programs
        let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: pending_reward,
        })?;
        

        Ok(Response::new()
            .add_submessages(cw20_res.messages)
            .add_attribute("action", "claim_rewards")
            .add_attribute("program_id", program_id)
            .add_attribute("user", info.sender.to_string())
            .add_attribute("amount", pending_reward.to_string()))
    } else {
        return Err(ContractError::ProgramNotFound {});
    }
}

fn execute_distribute_performance_fees(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only the authorized address can distribute performance fees
    if info.sender != config.fee_distributor {
        return Err(ContractError::Unauthorized {});
    }
let cw20_contract = deps.api.addr_validate(&config.liquidity_pool_contract.clone().to_string())?;
    // Get the total liquidity in the pool
    let pool_info: PoolInfo = deps.querier.query_wasm_smart(
        config.liquidity_pool_contract.clone(),
        &QueryMsg::GetPoolInfo {},
    )?;
    let total_liquidity = pool_info.total_liquidity;

    // Iterate over all liquidity providers and distribute the fees proportionally
    let liquidity_providers: Vec<(Addr, Uint128)> = LIQUIDITY_PROVIDERS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    for (provider, liquidity) in liquidity_providers {
        let share = liquidity.multiply_ratio(Uint128::from(1u128), total_liquidity);
        let fee_amount = share * amount;

        // Transfer the fees to the liquidity provider
        let cw20_contract = deps.api.addr_validate(&config.liquidity_pool_contract.clone().to_string())?;
        let cw20_msg = Cw20ExecuteMsg::Transfer {
            recipient: provider.to_string(),
            amount: fee_amount,
        };
        //let cw20_res = cw20_execute(deps.branch(), env.clone(), cw20_msg, vec![], cw20_contract)?;
        //let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), cw20_msg)?;
        let cw20_res = cw20_execute(deps.branch(), env.clone(), info.clone(), Cw20ExecuteMsg::Transfer {
            recipient: provider.to_string(),
            amount: fee_amount,
        })?;
        

        // Update the total fees distributed
        //TOTAL_FEES_DISTRIBUTED.update(deps.storage, |total| Ok(total + fee_amount))?;
        TOTAL_FEES_DISTRIBUTED.update(deps.storage, |total| -> StdResult<_> { Ok(total + fee_amount) })?;
    }

    Ok(Response::new()
        .add_attribute("action", "distribute_performance_fees")
        .add_attribute("amount", amount.to_string()))
}

fn execute_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&msg.msg)? {
        ExecuteMsg::Stake { program_id, amount } => execute_stake(deps, env, info, program_id, amount),
        _ => Err(ContractError::InvalidCw20ReceiveMsg {}),
    }
}

//#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::YieldFarmingProgram { program_id } => to_binary(&query_yield_farming_program(deps, program_id)?),
        QueryMsg::LiquidityMiningProgram { program_id } => to_binary(&query_liquidity_mining_program(deps, program_id)?),
        QueryMsg::UserStakedAmount { user, program_id } => to_binary(&query_user_staked_amount(deps, user, program_id)?),
        QueryMsg::UserRewardPerTokenPaid { user, program_id } => to_binary(&query_user_reward_per_token_paid(deps, user, program_id)?),
        QueryMsg::TotalFeesDistributed {} => to_binary(&query_total_fees_distributed(deps)?),
        QueryMsg::GetPoolInfo {} => to_binary(&query_pool_info(deps)?),


        QueryMsg::CnsteStakingProgram { program_id } => to_binary(&query_cnste_staking_program(deps, program_id)?),
        QueryMsg::UserCnsteStakedAmount { user, program_id } => to_binary(&query_user_cnste_staked_amount(deps, user, program_id)?),
        QueryMsg::TotalCnsteStakedAmount { program_id } => to_binary(&query_total_cnste_staked_amount(deps, program_id)?),
        QueryMsg::CnsteRewardPool {} => to_binary(&query_cnste_reward_pool(deps)?),

        
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_yield_farming_program(deps: Deps, program_id: String) -> StdResult<YieldFarmingProgram> {
    let program = YIELD_FARMING_PROGRAMS.load(deps.storage, program_id)?;
    Ok(program)
}

fn query_liquidity_mining_program(deps: Deps, program_id: String) -> StdResult<LiquidityMiningProgram> {
    let program = LIQUIDITY_MINING_PROGRAMS.load(deps.storage, program_id)?;
    Ok(program)
}

fn query_user_staked_amount(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let staked_amount = USER_STAKED_AMOUNT.load(deps.storage, (user_addr, program_id)).unwrap_or_default();
    Ok(staked_amount)
}

fn query_user_reward_per_token_paid(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let reward_per_token_paid = USER_REWARD_PER_TOKEN_PAID.load(deps.storage, (user_addr, program_id)).unwrap_or_default();
    Ok(reward_per_token_paid)
}

fn query_total_fees_distributed(deps: Deps) -> StdResult<Uint128> {
    let total_fees_distributed = TOTAL_FEES_DISTRIBUTED.load(deps.storage)?;
    Ok(total_fees_distributed)
}

fn query_pool_info(deps: Deps) -> StdResult<PoolInfo> {
    let config = CONFIG.load(deps.storage)?;
    let liquidity_pool_contract = config.liquidity_pool_contract;

    // Query the liquidity pool for the current pool composition
    let pool_info: PoolInfo = deps.querier.query_wasm_smart(
        liquidity_pool_contract,
        &QueryMsg::GetPoolInfo {},
    )?;

    Ok(pool_info)
}



fn query_cnste_staking_program(deps: Deps, program_id: String) -> StdResult<CnsteStakingProgramInfo> {
    let program = CNSTE_STAKING_PROGRAMS.load(deps.storage, program_id)?;
    Ok(CnsteStakingProgramInfo {
        program_id: program.program_id,
        reward_per_token_stored: program.reward_per_token_stored,
        last_update_time: program.last_update_time,
    })
}

fn query_user_cnste_staked_amount(deps: Deps, user: String, program_id: String) -> StdResult<Uint128> {
    let user_addr = deps.api.addr_validate(&user)?;
    let staked_amount = USER_CNSTE_STAKED_AMOUNT.load(deps.storage, (user_addr, program_id)).unwrap_or_default();
    Ok(staked_amount)
}

fn query_total_cnste_staked_amount(deps: Deps, program_id: String) -> StdResult<Uint128> {
    let total_staked_amount = TOTAL_CNSTE_STAKED_AMOUNT.load(deps.storage, program_id).unwrap_or_default();
    Ok(total_staked_amount)
}

fn query_cnste_reward_pool(deps: Deps) -> StdResult<Uint128> {
    let cnste_reward_pool = CNSTE_REWARD_POOL.load(deps.storage)?;
    Ok(cnste_reward_pool)
}




/*

To avoid or resolve the duplicate symbol errors, you should remove the #[entry_point] attributes from the query, execute, 
and instantiate functions in the cw20_base library that you are using in your incentive manager contract.

The cw20_base library already defines the entry points for these functions, so you don't need to redefine them in your contract. Instead, you should use the cw20_base library's implementations of these functions and build upon them.

Here's how you can modify your code to resolve the errors:

Remove the #[entry_point] attributes from the query, execute, and instantiate functions in your incentive manager contract.
Use the cw20_base library's implementations of these functions by calling them directly or by using the provided helper functions.

For example, in your execute function, you can use the cw20_base::contract::execute function to handle the standard CW20 messages 
and then add your custom message handling logic:
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            cw20_base::contract::execute_transfer(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Burn { amount } => {
            cw20_base::contract::execute_burn(deps, env, info, amount)
        }
        // Handle your custom messages here
        ExecuteMsg::CreateYieldFarmingProgram { ... } => {
            execute_create_yield_farming_program(deps, env, info, ...)
        }
        // ...
    }
}
Similarly, for the query and instantiate functions, you can use the corresponding functions from the cw20_base library and add your custom logic as needed.

Make sure that your custom message types (ExecuteMsg, QueryMsg, etc.) are defined correctly and don't conflict with the message types from the cw20_base library.
.................

*/