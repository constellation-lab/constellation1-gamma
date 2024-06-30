import { useQuery, UseQueryOptions } from "react-query";
import {
  ExecuteMsg,
  QueryMsg,
  Config,
  CalculateOptionPriceResponse,
  OptionPrice,
  PoolInfo,
  Uint128,
} from "./pricing_oracle.types";
import { PricingOracleQueryClient } from "./pricing_oracle.client";

export interface PricingOracleReactQuery<TData> {
  client: PricingOracleQueryClient;
  options?: UseQueryOptions<TData, Error, TData>;
}

export function usePricingOracleConfigQuery<TData = Config>({ client, options }: PricingOracleReactQuery<TData>) {
  return useQuery<Config, Error, TData>(
    ["pricingOracleConfig", client.contractAddress],
    () => client.config(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function usePricingOracleCalculateOptionPriceQuery<TData = CalculateOptionPriceResponse>({
  client,
  options,
  option_id,
  collateral,
  counter_offer,
  expiration,
}: PricingOracleReactQuery<TData> & {
  option_id: number;
  collateral: Uint128;
  counter_offer: Uint128;
  expiration: number;
}) {
  return useQuery<CalculateOptionPriceResponse, Error, TData>(
    ["pricingOracleCalculateOptionPrice", client.contractAddress, option_id, collateral, counter_offer, expiration],
    () =>
      client.calculateOptionPrice({
        option_id,
        collateral,
        counter_offer,
        expiration,
      }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function usePricingOracleGetOptionPriceQuery<TData = OptionPrice>({
  client,
  options,
  option_id,
}: PricingOracleReactQuery<TData> & { option_id: number }) {
  return useQuery<OptionPrice, Error, TData>(
    ["pricingOracleGetOptionPrice", client.contractAddress, option_id],
    () => client.getOptionPrice({ option_id }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function usePricingOracleGetPoolInfoQuery<TData = PoolInfo>({ client, options }: PricingOracleReactQuery<TData>) {
  return useQuery<PoolInfo, Error, TData>(
    ["pricingOracleGetPoolInfo", client.contractAddress],
    () => client.getPoolInfo(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}