import { useQuery, UseQueryOptions } from "react-query";
import {
  InstantiateMsg,
  ExecuteMsg,
  QueryMsg,
  LiquidityPool,
  LiquidityProviderInfo,
} from "./liquidity_pool.types";
import { LiquidityPoolQueryClient } from "./liquidity_pool.client";

export interface LiquidityPoolReactQuery<TData = LiquidityPool> {
  client: LiquidityPoolQueryClient;
  options?: UseQueryOptions<LiquidityPool, Error, TData>;
}

export function useLiquidityPoolGetPoolQuery<TData = LiquidityPool>({
  client,
  options,
}: LiquidityPoolReactQuery<TData>) {
  return useQuery<LiquidityPool, Error, TData>(
    ["liquidityPoolGetPool", client.contractAddress],
    () => client.getPool(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export interface LiquidityPoolGetLiquidityProviderInfoQuery<TData = LiquidityProviderInfo>
  extends LiquidityPoolReactQuery<TData> {
  variables: { address: string };
}

export function useLiquidityPoolGetLiquidityProviderInfoQuery<TData = LiquidityProviderInfo>({
  client,
  options,
  variables: { address },
}: LiquidityPoolGetLiquidityProviderInfoQuery<TData>) {
  return useQuery<LiquidityProviderInfo, Error, TData>(
    ["liquidityPoolGetLiquidityProviderInfo", client.contractAddress, address],
    () => client.getLiquidityProviderInfo({ address }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}