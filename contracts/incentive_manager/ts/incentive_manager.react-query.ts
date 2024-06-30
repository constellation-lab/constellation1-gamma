import { useQuery, UseQueryOptions } from "react-query";
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

export interface IncentiveManagerReactQuery<TData> {
  client: IncentiveManagerQueryClient;
  options?: UseQueryOptions<TData, Error, TData>;
}

export function useIncentiveManagerConfigQuery<TData = Config>({
  client,
  options,
}: IncentiveManagerReactQuery<TData>) {
  return useQuery<Config, Error, TData>(
    ["incentiveManagerConfig", client.contractAddress],
    () => client.config(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerYieldFarmingProgramQuery<TData = YieldFarmingProgram>({
  client,
  options,
  program_id,
}: IncentiveManagerReactQuery<TData> & { program_id: string }) {
  return useQuery<YieldFarmingProgram, Error, TData>(
    ["incentiveManagerYieldFarmingProgram", client.contractAddress, program_id],
    () => client.yieldFarmingProgram({ program_id }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerLiquidityMiningProgramQuery<TData = LiquidityMiningProgram>({
  client,
  options,
  program_id,
}: IncentiveManagerReactQuery<TData> & { program_id: string }) {
  return useQuery<LiquidityMiningProgram, Error, TData>(
    ["incentiveManagerLiquidityMiningProgram", client.contractAddress, program_id],
    () => client.liquidityMiningProgram({ program_id }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerUserStakedAmountQuery<TData = Uint128>({
  client,
  options,
  program_id,
  user,
}: IncentiveManagerReactQuery<TData> & { program_id: string; user: string }) {
  return useQuery<Uint128, Error, TData>(
    ["incentiveManagerUserStakedAmount", client.contractAddress, program_id, user],
    () => client.userStakedAmount({ program_id, user }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerUserRewardPerTokenPaidQuery<TData = Uint128>({
  client,
  options,
  program_id,
  user,
}: IncentiveManagerReactQuery<TData> & { program_id: string; user: string }) {
  return useQuery<Uint128, Error, TData>(
    ["incentiveManagerUserRewardPerTokenPaid", client.contractAddress, program_id, user],
    () => client.userRewardPerTokenPaid({ program_id, user }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerTotalFeesDistributedQuery<TData = Uint128>({
  client,
  options,
}: IncentiveManagerReactQuery<TData>) {
  return useQuery<Uint128, Error, TData>(
    ["incentiveManagerTotalFeesDistributed", client.contractAddress],
    () => client.totalFeesDistributed(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerGetPoolInfoQuery<TData = PoolInfo>({
  client,
  options,
}: IncentiveManagerReactQuery<TData>) {
  return useQuery<PoolInfo, Error, TData>(
    ["incentiveManagerGetPoolInfo", client.contractAddress],
    () => client.getPoolInfo(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerCnsteStakingProgramQuery<TData = CnsteStakingProgramInfo>({
  client,
  options,
  program_id,
}: IncentiveManagerReactQuery<TData> & { program_id: string }) {
  return useQuery<CnsteStakingProgramInfo, Error, TData>(
    ["incentiveManagerCnsteStakingProgram", client.contractAddress, program_id],
    () => client.cnsteStakingProgram({ program_id }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerUserCnsteStakedAmountQuery<TData = Uint128>({
  client,
  options,
  program_id,
  user,
}: IncentiveManagerReactQuery<TData> & { program_id: string; user: string }) {
  return useQuery<Uint128, Error, TData>(
    ["incentiveManagerUserCnsteStakedAmount", client.contractAddress, program_id, user],
    () => client.userCnsteStakedAmount({ program_id, user }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerTotalCnsteStakedAmountQuery<TData = Uint128>({
  client,
  options,
  program_id,
}: IncentiveManagerReactQuery<TData> & { program_id: string }) {
  return useQuery<Uint128, Error, TData>(
    ["incentiveManagerTotalCnsteStakedAmount", client.contractAddress, program_id],
    () => client.totalCnsteStakedAmount({ program_id }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useIncentiveManagerCnsteRewardPoolQuery<TData = Uint128>({
  client,
  options,
}: IncentiveManagerReactQuery<TData>) {
  return useQuery<Uint128, Error, TData>(
    ["incentiveManagerCnsteRewardPool", client.contractAddress],
    () => client.cnsteRewardPool(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}