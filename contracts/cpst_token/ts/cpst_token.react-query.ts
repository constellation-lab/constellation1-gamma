import { useQuery, UseQueryOptions } from "react-query";
import {
  ExecuteMsg,
  QueryMsg,
  Uint128,
  Binary,
  Expiration,
  Timestamp,
  Uint64,
} from "./cpst_token.types";
import { CpstTokenQueryClient } from "./cpst_token.client";

export interface CpstTokenReactQuery<TData> {
  client: CpstTokenQueryClient;
  options?: UseQueryOptions<TData, Error, TData>;
}

export function useCpstTokenBalanceQuery<TData = Uint128>({
  client,
  options,
  address,
}: CpstTokenReactQuery<TData> & { address: string }) {
  return useQuery<Uint128, Error, TData>(
    ["cpstTokenBalance", client.contractAddress, address],
    () => client.balance({ address }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCpstTokenTokenInfoQuery<TData = unknown>({
  client,
  options,
}: CpstTokenReactQuery<TData>) {
  return useQuery<unknown, Error, TData>(
    ["cpstTokenTokenInfo", client.contractAddress],
    () => client.tokenInfo(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCpstTokenAllowanceQuery<TData = Uint128>({
  client,
  options,
  owner,
  spender,
}: CpstTokenReactQuery<TData> & { owner: string; spender: string }) {
  return useQuery<Uint128, Error, TData>(
    ["cpstTokenAllowance", client.contractAddress, owner, spender],
    () => client.allowance({ owner, spender }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCpstTokenAllAllowancesQuery<TData = unknown>({
  client,
  options,
  owner,
  startAfter,
  limit,
}: CpstTokenReactQuery<TData> & { owner: string; startAfter?: string; limit?: number }) {
  return useQuery<unknown, Error, TData>(
    ["cpstTokenAllAllowances", client.contractAddress, owner, startAfter, limit],
    () => client.allAllowances({ owner, startAfter, limit }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCpstTokenAllAccountsQuery<TData = unknown>({
  client,
  options,
  startAfter,
  limit,
}: CpstTokenReactQuery<TData> & { startAfter?: string; limit?: number }) {
  return useQuery<unknown, Error, TData>(
    ["cpstTokenAllAccounts", client.contractAddress, startAfter, limit],
    () => client.allAccounts({ startAfter, limit }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCpstTokenMinterQuery<TData = unknown>({ client, options }: CpstTokenReactQuery<TData>) {
  return useQuery<unknown, Error, TData>(
    ["cpstTokenMinter", client.contractAddress],
    () => client.minter(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCpstTokenMarketingInfoQuery<TData = unknown>({ client, options }: CpstTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cpstTokenMarketingInfo", client.contractAddress],
      () => client.marketingInfo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCpstTokenDownloadLogoQuery<TData = unknown>({ client, options }: CpstTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cpstTokenDownloadLogo", client.contractAddress],
      () => client.downloadLogo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }