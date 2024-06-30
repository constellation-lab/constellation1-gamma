import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import {
  ExecuteMsg,
  QueryMsg,
  Config,
  YieldFarmingProgram,
  LiquidityMiningProgram,
  Uint128,
  PoolInfo,
  CnsteStakingProgramInfo,
} from "./incentive_manager.types";
import { IncentiveManagerQueryClient } from "./incentive_manager.client";

type QueryClientParams = {
  contractAddress: string;
};

export const incentiveManagerQueryClient = selectorFamily<IncentiveManagerQueryClient, QueryClientParams>({
  key: "incentiveManagerQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new IncentiveManagerQueryClient(client, contractAddress);
  },
});

export const incentiveManagerConfigSelector = selectorFamily<Config, QueryClientParams>({
  key: "incentiveManagerConfig",
  get: (queryClientParams) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.config();
  },
});

export const incentiveManagerYieldFarmingProgramSelector = selectorFamily
  YieldFarmingProgram,
  QueryClientParams & { program_id: string }
>({
  key: "incentiveManagerYieldFarmingProgram",
  get: ({ program_id, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.yieldFarmingProgram({ program_id });
  },
});

export const incentiveManagerLiquidityMiningProgramSelector = selectorFamily
  LiquidityMiningProgram,
  QueryClientParams & { program_id: string }
>({
  key: "incentiveManagerLiquidityMiningProgram",
  get: ({ program_id, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.liquidityMiningProgram({ program_id });
  },
});

export const incentiveManagerUserStakedAmountSelector = selectorFamily
  Uint128,
  QueryClientParams & { program_id: string; user: string }
>({
  key: "incentiveManagerUserStakedAmount",
  get: ({ program_id, user, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.userStakedAmount({ program_id, user });
  },
});

export const incentiveManagerUserRewardPerTokenPaidSelector = selectorFamily
  Uint128,
  QueryClientParams & { program_id: string; user: string }
>({
  key: "incentiveManagerUserRewardPerTokenPaid",
  get: ({ program_id, user, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.userRewardPerTokenPaid({ program_id, user });
  },
});

export const incentiveManagerTotalFeesDistributedSelector = selectorFamily<Uint128, QueryClientParams>({
  key: "incentiveManagerTotalFeesDistributed",
  get: (queryClientParams) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.totalFeesDistributed();
  },
});

export const incentiveManagerGetPoolInfoSelector = selectorFamily<PoolInfo, QueryClientParams>({
  key: "incentiveManagerGetPoolInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.getPoolInfo();
  },
});

export const incentiveManagerCnsteStakingProgramSelector = selectorFamily
  CnsteStakingProgramInfo,
  QueryClientParams & { program_id: string }
>({
  key: "incentiveManagerCnsteStakingProgram",
  get: ({ program_id, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.cnsteStakingProgram({ program_id });
  },
});

export const incentiveManagerUserCnsteStakedAmountSelector = selectorFamily
  Uint128,
  QueryClientParams & { program_id: string; user: string }
>({
  key: "incentiveManagerUserCnsteStakedAmount",
  get: ({ program_id, user, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.userCnsteStakedAmount({ program_id, user });
  },
});

export const incentiveManagerTotalCnsteStakedAmountSelector = selectorFamily
  Uint128,
  QueryClientParams & { program_id: string }
>({
  key: "incentiveManagerTotalCnsteStakedAmount",
  get: ({ program_id, ...queryClientParams }) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.totalCnsteStakedAmount({ program_id });
  },
});

export const incentiveManagerCnsteRewardPoolSelector = selectorFamily<Uint128, QueryClientParams>({
  key: "incentiveManagerCnsteRewardPool",
  get: (queryClientParams) => ({ get }) => {
    const client = get(incentiveManagerQueryClient(queryClientParams));
    return client.cnsteRewardPool();
  },
});