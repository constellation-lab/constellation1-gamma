export interface InstantiateMsg {
    cdt_token_contract: string;
    cnste_token_contract: string;
    constella_option_contract: string;
    cpst_token_contract: string;
    fee_distributor: string;
    governance_token: string;
    liquidity_pool_contract: string;
    option_marketplace_contract: string;
    pricing_oracle_contract: string;
  }
  
  export type ExecuteMsg =
    | {
        create_yield_farming_program: {
          end_time: number;
          program_id: string;
          reward_rate: Uint128;
          reward_token: string;
          start_time: number;
        };
      }
    | {
        create_liquidity_mining_program: {
          end_time: number;
          option_pair: string;
          program_id: string;
          reward_multiplier: Uint128;
          start_time: number;
        };
      }
    | {
        create_cnste_staking_program: {
          last_update_time: number;
          program_id: string;
          reward_per_token_stored: Uint128;
        };
      }
    | {
        stake: {
          amount: Uint128;
          program_id: string;
        };
      }
    | {
        unstake: {
          amount: Uint128;
          program_id: string;
        };
      }
    | {
        claim_rewards: {
          program_id: string;
        };
      }
    | {
        distribute_performance_fees: {
          amount: Uint128;
        };
      }
    | {
        receive: Cw20ReceiveMsg;
      };
  
  export interface Cw20ReceiveMsg {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }
  
  export type Binary = string;
  export type Uint128 = string;
  
  export type QueryMsg =
    | {
        config: {};
      }
    | {
        yield_farming_program: {
          program_id: string;
        };
      }
    | {
        liquidity_mining_program: {
          program_id: string;
        };
      }
    | {
        user_staked_amount: {
          program_id: string;
          user: string;
        };
      }
    | {
        user_reward_per_token_paid: {
          program_id: string;
          user: string;
        };
      }
    | {
        total_fees_distributed: {};
      }
    | {
        get_pool_info: {};
      }
    | {
        cnste_staking_program: {
          program_id: string;
        };
      }
    | {
        user_cnste_staked_amount: {
          program_id: string;
          user: string;
        };
      }
    | {
        total_cnste_staked_amount: {
          program_id: string;
        };
      }
    | {
        cnste_reward_pool: {};
      };
  
  export interface Config {
    cdt_token_contract: Addr;
    cnste_token_contract: Addr;
    constella_option_contract: Addr;
    cpst_token_contract: Addr;
    fee_distributor: Addr;
    governance_token: Addr;
    incentive_manager_contract: Addr;
    liquidity_pool_contract: Addr;
    option_marketplace_contract: Addr;
    pricing_oracle_contract: Addr;
  }
  
  export type Addr = string;
  
  export interface YieldFarmingProgram {
    end_time: number;
    last_update_time: number;
    program_id: string;
    reward_per_token_stored: Uint128;
    reward_rate: Uint128;
    reward_token: string;
    start_time: number;
    total_staked: Uint128;
  }
  
  export interface LiquidityMiningProgram {
    end_time: number;
    last_update_time: number;
    option_pair: string;
    program_id: string;
    reward_multiplier: Uint128;
    reward_per_token_stored: Uint128;
    start_time: number;
    total_liquidity: Uint128;
  }
  
  export interface PoolInfo {
    total_liquidity: Uint128;
  }
  
  export interface CnsteStakingProgramInfo {
    last_update_time: number;
    program_id: string;
    reward_per_token_stored: Uint128;
  }