import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import {
  InstantiateMsg,
  ExecuteMsg,
  QueryMsg,
  Config,
  YieldFarmingProgram,
  LiquidityMiningProgram,
  Uint128,
  PoolInfo,
  CnsteStakingProgramInfo,
} from "./incentive_manager.types";

export interface IncentiveManagerReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  yieldFarmingProgram: ({ program_id }: { program_id: string }) => Promise<YieldFarmingProgram>;
  liquidityMiningProgram: ({
    program_id,
  }: {
    program_id: string;
  }) => Promise<LiquidityMiningProgram>;
  userStakedAmount: ({
    program_id,
    user,
  }: {
    program_id: string;
    user: string;
  }) => Promise<Uint128>;
  userRewardPerTokenPaid: ({
    program_id,
    user,
  }: {
    program_id: string;
    user: string;
  }) => Promise<Uint128>;
  totalFeesDistributed: () => Promise<Uint128>;
  getPoolInfo: () => Promise<PoolInfo>;
  cnsteStakingProgram: ({ program_id }: { program_id: string }) => Promise<CnsteStakingProgramInfo>;
  userCnsteStakedAmount: ({
    program_id,
    user,
  }: {
    program_id: string;
    user: string;
  }) => Promise<Uint128>;
  totalCnsteStakedAmount: ({ program_id }: { program_id: string }) => Promise<Uint128>;
  cnsteRewardPool: () => Promise<Uint128>;
}

export class IncentiveManagerQueryClient implements IncentiveManagerReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.yieldFarmingProgram = this.yieldFarmingProgram.bind(this);
    this.liquidityMiningProgram = this.liquidityMiningProgram.bind(this);
    this.userStakedAmount = this.userStakedAmount.bind(this);
    this.userRewardPerTokenPaid = this.userRewardPerTokenPaid.bind(this);
    this.totalFeesDistributed = this.totalFeesDistributed.bind(this);
    this.getPoolInfo = this.getPoolInfo.bind(this);
    this.cnsteStakingProgram = this.cnsteStakingProgram.bind(this);
    this.userCnsteStakedAmount = this.userCnsteStakedAmount.bind(this);
    this.totalCnsteStakedAmount = this.totalCnsteStakedAmount.bind(this);
    this.cnsteRewardPool = this.cnsteRewardPool.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {},
    });
  };

  yieldFarmingProgram = async ({
    program_id,
  }: {
    program_id: string;
  }): Promise<YieldFarmingProgram> => {
    return this.client.queryContractSmart(this.contractAddress, {
      yield_farming_program: { program_id },
    });
  };

  liquidityMiningProgram = async ({
    program_id,
  }: {
    program_id: string;
  }): Promise<LiquidityMiningProgram> => {
    return this.client.queryContractSmart(this.contractAddress, {
      liquidity_mining_program: { program_id },
    });
  };

  userStakedAmount = async ({
    program_id,
    user,
  }: {
    program_id: string;
    user: string;
  }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      user_staked_amount: { program_id, user },
    });
  };

  userRewardPerTokenPaid = async ({
    program_id,
    user,
  }: {
    program_id: string;
    user: string;
  }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      user_reward_per_token_paid: { program_id, user },
    });
  };

  totalFeesDistributed = async (): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      total_fees_distributed: {},
    });
  };

  getPoolInfo = async (): Promise<PoolInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_pool_info: {},
    });
  };

  cnsteStakingProgram = async ({
    program_id,
  }: {
    program_id: string;
  }): Promise<CnsteStakingProgramInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      cnste_staking_program: { program_id },
    });
  };

  userCnsteStakedAmount = async ({
    program_id,
    user,
  }: {
    program_id: string;
    user: string;
  }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      user_cnste_staked_amount: { program_id, user },
    });
  };

  totalCnsteStakedAmount = async ({
    program_id,
  }: {
    program_id: string;
  }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      total_cnste_staked_amount: { program_id },
    });
  };

  cnsteRewardPool = async (): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      cnste_reward_pool: {},
    });
  };
}

export interface IncentiveManagerInterface extends IncentiveManagerReadOnlyInterface {
  contractAddress: string;
  sender: string;

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
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

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
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

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
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  stake: (
    {
      amount,
      program_id,
    }: {
      amount: Uint128;
      program_id: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  unstake: (
    {
      amount,
      program_id,
    }: {
      amount: Uint128;
      program_id: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  claimRewards: (
    {
      program_id,
    }: {
      program_id: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  distributePerformanceFees: (
    {
      amount,
    }: {
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}

export class IncentiveManagerClient
  extends IncentiveManagerQueryClient
  implements IncentiveManagerInterface
{
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createYieldFarmingProgram = this.createYieldFarmingProgram.bind(this);
    this.createLiquidityMiningProgram = this.createLiquidityMiningProgram.bind(this);
    this.createCnsteStakingProgram = this.createCnsteStakingProgram.bind(this);
    this.stake = this.stake.bind(this);
    this.unstake = this.unstake.bind(this);
    this.claimRewards = this.claimRewards.bind(this);
    this.distributePerformanceFees = this.distributePerformanceFees.bind(this);
  }

  createYieldFarmingProgram = async (
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
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        create_yield_farming_program: {
          end_time,
          program_id,
          reward_rate,
          reward_token,
          start_time,
        },
      },
      fee,
      memo,
      funds
    );
};

createLiquidityMiningProgram = async (
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
  fee: number | StdFee | "auto" = "auto",
  memo?: string,
  funds?: Coin[]
): Promise<ExecuteResult> => {
  return await this.client.execute(
    this.sender,
    this.contractAddress,
    {
      create_liquidity_mining_program: {
        end_time,
        option_pair,
        program_id,
        reward_multiplier,
        start_time,
      },
    },
    fee,
    memo,
    funds
  );
};

createCnsteStakingProgram = async (
  {
    last_update_time,
    program_id,
    reward_per_token_stored,
  }: {
    last_update_time: number;
    program_id: string;
    reward_per_token_stored: Uint128;
  },
  fee: number | StdFee | "auto" = "auto",
  memo?: string,
  funds?: Coin[]
): Promise<ExecuteResult> => {
  return await this.client.execute(
    this.sender,
    this.contractAddress,
    {
      create_cnste_staking_program: {
        last_update_time,
        program_id,
        reward_per_token_stored,
      },
    },
    fee,
    memo,
    funds
  );
};

stake = async (
  {
    amount,
    program_id,
  }: {
    amount: Uint128;
    program_id: string;
  },
  fee: number | StdFee | "auto" = "auto",
  memo?: string,
  funds?: Coin[]
): Promise<ExecuteResult> => {
  return await this.client.execute(
    this.sender,
    this.contractAddress,
    {
      stake: {
        amount,
        program_id,
      },
    },
    fee,
    memo,
    funds
  );
};

unstake = async (
  {
    amount,
    program_id,
  }: {
    amount: Uint128;
    program_id: string;
  },
  fee: number | StdFee | "auto" = "auto",
  memo?: string,
  funds?: Coin[]
): Promise<ExecuteResult> => {
  return await this.client.execute(
    this.sender,
    this.contractAddress,
    {
      unstake: {
        amount,
        program_id,
      },
    },
    fee,
    memo,
    funds
  );
};

claimRewards = async (
  {
    program_id,
  }: {
    program_id: string;
  },
  fee: number | StdFee | "auto" = "auto",
  memo?: string,
  funds?: Coin[]
): Promise<ExecuteResult> => {
  return await this.client.execute(
    this.sender,
    this.contractAddress,
    {
      claim_rewards: {
        program_id,
      },
    },
    fee,
    memo,
    funds
  );
};

distributePerformanceFees = async (
  {
    amount,
  }: {
    amount: Uint128;
  },
  fee: number | StdFee | "auto" = "auto",
  memo?: string,
  funds?: Coin[]
): Promise<ExecuteResult> => {
  return await this.client.execute(
    this.sender,
    this.contractAddress,
    {
      distribute_performance_fees: {
        amount,
      },
    },
    fee,
    memo,
    funds
  );
};
}