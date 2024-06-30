import { useQuery, UseQueryOptions } from "react-query";
import { ExecuteMsg, QueryMsg, Option, Uint128, Decimal } from "./option_marketplace.types";
import { OptionMarketplaceQueryClient } from "./option_marketplace.client";

export interface OptionMarketplaceReactQuery<TData = Option> {
  client: OptionMarketplaceQueryClient;
  options?: UseQueryOptions<Option, Error, TData>;
}

export function useOptionMarketplaceGetOptionQuery<TData = Option>({
  client,
  options,
  option_id,
}: OptionMarketplaceReactQuery<TData> & { option_id: number }) {
  return useQuery<Option, Error, TData>(
    ["optionMarketplaceGetOption", client.contractAddress, option_id],
    () => client.getOption({ option_id }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useOptionMarketplaceListOptionsQuery<TData = Option[]>({
  client,
  options,
}: OptionMarketplaceReactQuery<TData>) {
  return useQuery<Option[], Error, TData>(
    ["optionMarketplaceListOptions", client.contractAddress],
    () => client.listOptions(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useOptionMarketplaceGetOptionPriceQuery<TData = Uint128>({
  client,
  options,
  option_id,
  slippage_tolerance,
}: OptionMarketplaceReactQuery<TData> & {
  option_id: number;
  slippage_tolerance: Decimal;
}) {
  return useQuery<Uint128, Error, TData>(
    ["optionMarketplaceGetOptionPrice", client.contractAddress, option_id, slippage_tolerance],
    () => client.getOptionPrice({ option_id, slippage_tolerance }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}