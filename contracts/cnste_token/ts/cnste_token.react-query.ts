import { useQuery, UseQueryOptions } from "react-query";
import {
  ExecuteMsg,
  QueryMsg,
  StakeInfo,
  Uint128,
  Binary,
} from "./cnste_token.types";
import { CnsteTokenQueryClient } from "./cnste_token.client";

export interface CnsteTokenReactQuery<TData> {
  client: CnsteTokenQueryClient;
  options?: UseQueryOptions<TData, Error, TData>;
}

export function useCnsteTokenBalanceQuery<TData = Uint128>({
  client,
  options,
  address,
}: CnsteTokenReactQuery<TData> & { address: string }) {
  return useQuery<Uint128, Error, TData>(
    ["cnsteTokenBalance", client.contractAddress, address],
    () => client.balance({ address }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),}
    );
  }
  
  export function useCnsteTokenTokenInfoQuery<TData = unknown>({
    client,
    options,
  }: CnsteTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cnsteTokenTokenInfo", client.contractAddress],
      () => client.tokenInfo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenAllowanceQuery<TData = Uint128>({
    client,
    options,
    owner,
    spender,
  }: CnsteTokenReactQuery<TData> & { owner: string; spender: string }) {
    return useQuery<Uint128, Error, TData>(
      ["cnsteTokenAllowance", client.contractAddress, owner, spender],
      () => client.allowance({ owner, spender }),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenAllAllowancesQuery<TData = unknown>({
    client,
    options,
    owner,
    startAfter,
    limit,
  }: CnsteTokenReactQuery<TData> & { owner: string; startAfter?: string; limit?: number }) {
    return useQuery<unknown, Error, TData>(
      ["cnsteTokenAllAllowances", client.contractAddress, owner, startAfter, limit],
      () => client.allAllowances({ owner, startAfter, limit }),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenAllAccountsQuery<TData = unknown>({
    client,
    options,
    startAfter,
    limit,
  }: CnsteTokenReactQuery<TData> & { startAfter?: string; limit?: number }) {
    return useQuery<unknown, Error, TData>(
      ["cnsteTokenAllAccounts", client.contractAddress, startAfter, limit],
      () => client.allAccounts({ startAfter, limit }),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenMinterQuery<TData = unknown>({ client, options }: CnsteTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cnsteTokenMinter", client.contractAddress],
      () => client.minter(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenMarketingInfoQuery<TData = unknown>({ client, options }: CnsteTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cnsteTokenMarketingInfo", client.contractAddress],
      () => client.marketingInfo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenDownloadLogoQuery<TData = unknown>({ client, options }: CnsteTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cnsteTokenDownloadLogo", client.contractAddress],
      () => client.downloadLogo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCnsteTokenStakeInfoQuery<TData = StakeInfo>({
    client,
    options,
    address,
  }: CnsteTokenReactQuery<TData> & { address: string }) {
    return useQuery<StakeInfo, Error, TData>(
      ["cnsteTokenStakeInfo", client.contractAddress, address],
      () => client.stakeInfo({ address }),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }