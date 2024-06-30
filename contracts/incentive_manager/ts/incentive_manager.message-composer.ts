import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import {
  ExecuteMsg,
  Uint128,
  Binary,
  Cw20ReceiveMsg,
} from "./incentive_manager.types";

export interface IncentiveManagerMessageComposer {
  sender: string;
  contractAddress: string;

  createYieldFarmingProgram: (
    {
      end_time,
      program_id,
      reward_rate,
      reward_token,
      start_time,
    }: {
      end_time: number;
      program_id: string;
      reward_rate: Uint128;
      reward_token: string;
      start_time: number;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  createLiquidityMiningProgram: (
    {
      end_time,
      option_pair,
      program_id,
      reward_multiplier,
      start_time,
    }: {
      end_time: number;
      option_pair: string;
      program_id: string;
      reward_multiplier: Uint128;
      start_time: number;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  createCnsteStakingProgram: (
    {
      last_update_time,
      program_id,
      reward_per_token_stored,
    }: {
      last_update_time: number;
      program_id: string;
      reward_per_token_stored: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  stake: (
    {
      amount,
      program_id,
    }: {
      amount: Uint128;
      program_id: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  unstake: (
    {
      amount,
      program_id,
    }: {
      amount: Uint128;
      program_id: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  claimRewards: (
    {
      program_id,
    }: {
      program_id: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  distributePerformanceFees: (
    {
      amount,
    }: {
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  receive: (
    {
      amount,
      msg,
      sender,
    }: {
      amount: Uint128;
      msg: Binary;
      sender: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
}

export class IncentiveManagerMessageComposer implements IncentiveManagerMessageComposer {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createYieldFarmingProgram = this.createYieldFarmingProgram.bind(this);
    this.createLiquidityMiningProgram = this.createLiquidityMiningProgram.bind(this);
    this.createCnsteStakingProgram = this.createCnsteStakingProgram.bind(this);
    this.stake = this.stake.bind(this);
    this.unstake = this.unstake.bind(this);
    this.claimRewards = this.claimRewards.bind(this);
    this.distributePerformanceFees = this.distributePerformanceFees.bind(this);
    this.receive = this.receive.bind(this);
  }

  createYieldFarmingProgram = (
    {
      end_time,
      program_id,
      reward_rate,
      reward_token,
      start_time,
    }: {
      end_time: number;
      program_id: string;
      reward_rate: Uint128;
      reward_token: string;
      start_time: number;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            create_yield_farming_program: {
              end_time,
              program_id,
              reward_rate,
              reward_token,
              start_time,
            },
          })
        ),
        funds,
      }),
    };
  };

  createLiquidityMiningProgram = (
    {
      end_time,
      option_pair,
      program_id,
      reward_multiplier,
      start_time,
    }: {
      end_time: number;
      option_pair: string;
      program_id: string;
      reward_multiplier: Uint128;
      start_time: number;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            create_liquidity_mining_program: {
              end_time,
              option_pair,
              program_id,
              reward_multiplier,
              start_time,
            },
          })
        ),
        funds,
      }),
    };
  };

  createCnsteStakingProgram = (
    {
      last_update_time,
      program_id,
      reward_per_token_stored,
    }: {
      last_update_time: number;
      program_id: string;
      reward_per_token_stored: Uint128;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            create_cnste_staking_program: {
              last_update_time,
              program_id,
              reward_per_token_stored,
            },
          })
        ),
        funds,
      }),
    };
  };

  stake = (
    {
      amount,
      program_id,
    }: {
      amount: Uint128;
      program_id: string;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            stake: {
              amount,
              program_id,
            },
          })
        ),
        funds,
      }),
    };
  };

  unstake = (
    {
      amount,
      program_id,
    }: {
      amount: Uint128;
      program_id: string;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            unstake: {
              amount,
              program_id,
            },
          })
        ),
        funds,
      }),
    };
  };

  claimRewards = (
    {
      program_id,
    }: {
      program_id: string;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            claim_rewards: {
              program_id,
            },
          })
        ),
        funds,
      }),
    };
  };

  distributePerformanceFees = (
    {
      amount,
    }: {
      amount: Uint128;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            distribute_performance_fees: {
              amount,
            },
          })
        ),
        funds,
      }),
    };
  };

  receive = (
    {
      amount,
      msg,
      sender,
    }: {
      amount: Uint128;
      msg: Binary;
      sender: string;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            receive: {
              amount,
              msg,
              sender,
            },
          })
        ),
        funds,
      }),
    };
  };
}