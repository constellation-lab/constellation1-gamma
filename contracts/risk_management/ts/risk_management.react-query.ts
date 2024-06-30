import { useQuery, UseQueryOptions } from "react-query";
import {
  ExecuteMsg,
  QueryMsg,
  Config,
  PositionLimit,
  CircuitBreaker,
  PoolInfo,
  OptionPosition,
  Uint128,
  Decimal,
} from "./risk_management.types";
import { RiskManagementQueryClient } from "./risk_management.client";

export interface RiskManagementReactQuery<TData> {
  client: RiskManagementQueryClient;
  options?: UseQueryOptions<TData, Error, TData>;
}

export function useRiskManagementConfigQuery<TData = Config>({
  client,
  options,
}: RiskManagementReactQuery<TData>) {
  return useQuery<Config, Error, TData>(
    ["riskManagementConfig", client.contractAddress],
    () => client.config(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useRiskManagementPositionLimitQuery<TData = PositionLimit>({
  client,
  options,
  option_pair,
}: RiskManagementReactQuery<TData> & { option_pair: string }) {
  return useQuery<PositionLimit, Error, TData>(
    ["riskManagementPositionLimit", client.contractAddress, option_pair],
    () => client.positionLimit({ option_pair }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useRiskManagementCircuitBreakerQuery<TData = CircuitBreaker>({
  client,
  options,
  option_pair,
}: RiskManagementReactQuery<TData> & { option_pair: string }) {
  return useQuery<CircuitBreaker, Error, TData>(
    ["riskManagementCircuitBreaker", client.contractAddress, option_pair],
    () => client.circuitBreaker({ option_pair }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useRiskManagementGetPoolInfoQuery<TData = PoolInfo>({
  client,
  options,
}: RiskManagementReactQuery<TData>) {
  return useQuery<PoolInfo, Error, TData>(
    ["riskManagementGetPoolInfo", client.contractAddress],
    () => client.getPoolInfo(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useRiskManagementGetOptionPositionsQuery<TData = OptionPosition[]>({
  client,
  options,
}: RiskManagementReactQuery<TData>) {
  return useQuery<OptionPosition[], Error, TData>(
    ["riskManagementGetOptionPositions", client.contractAddress],
    () => client.getOptionPositions(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useRiskManagementGetPriceQuery<TData = Decimal>({
  client,
  options,
  option_pair,
}: RiskManagementReactQuery<TData> & { option_pair: string }) {
  return useQuery<Decimal, Error, TData>(
    ["riskManagementGetPrice", client.contractAddress, option_pair],
    () => client.getPrice({ option_pair }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}