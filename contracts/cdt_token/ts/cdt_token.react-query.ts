import { useQuery, UseQueryOptions } from "react-query";
import {
  ExecuteMsg,
  QueryMsg,
  VotingPowerResponse,
  Uint128,
  Binary,
} from "./cdt_token.types";
import { CdtTokenQueryClient } from "./cdt_token.client";

export interface CdtTokenReactQuery<TData> {
  client: CdtTokenQueryClient;
  options?: UseQueryOptions<TData, Error, TData>;
}

export function useCdtTokenBalanceQuery<TData = Uint128>({
  client,
  options,
  address,
}: CdtTokenReactQuery<TData> & { address: string }) {
  return useQuery<Uint128, Error, TData>(
    ["cdtTokenBalance", client.contractAddress, address],
    () => client.balance({ address }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCdtTokenTokenInfoQuery<TData = unknown>({
  client,
  options,
}: CdtTokenReactQuery<TData>) {
  return useQuery<unknown, Error, TData>(
    ["cdtTokenTokenInfo", client.contractAddress],
    () => client.tokenInfo(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCdtTokenAllowanceQuery<TData = Uint128>({
  client,
  options,
  owner,
  spender,
}: CdtTokenReactQuery<TData> & { owner: string; spender: string }) {
  return useQuery<Uint128, Error, TData>(
    ["cdtTokenAllowance", client.contractAddress, owner, spender],
    () => client.allowance({ owner, spender }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCdtTokenAllAllowancesQuery<TData = unknown>({
  client,
  options,
  owner,
  startAfter,
  limit,
}: CdtTokenReactQuery<TData> & { owner: string; startAfter?: string; limit?: number }) {
  return useQuery<unknown, Error, TData>(
    ["cdtTokenAllAllowances", client.contractAddress, owner, startAfter, limit],
    () => client.allAllowances({ owner, startAfter, limit }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCdtTokenAllAccountsQuery<TData = unknown>({
  client,
  options,
  startAfter,
  limit,
}: CdtTokenReactQuery<TData> & { startAfter?: string; limit?: number }) {
  return useQuery<unknown, Error, TData>(
    ["cdtTokenAllAccounts", client.contractAddress, startAfter, limit],
    () => client.allAccounts({ startAfter, limit }),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCdtTokenMinterQuery<TData = unknown>({ client, options }: CdtTokenReactQuery<TData>) {
  return useQuery<unknown, Error, TData>(
    ["cdtTokenMinter", client.contractAddress],
    () => client.minter(),
    {
      ...options,
      enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
    }
  );
}

export function useCdtTokenMarketingInfoQuery<TData = unknown>({ client, options }: CdtTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cdtTokenMarketingInfo", client.contractAddress],
      () => client.marketingInfo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCdtTokenDownloadLogoQuery<TData = unknown>({ client, options }: CdtTokenReactQuery<TData>) {
    return useQuery<unknown, Error, TData>(
      ["cdtTokenDownloadLogo", client.contractAddress],
      () => client.downloadLogo(),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }
  
  export function useCdtTokenVotingPowerQuery<TData = VotingPowerResponse>({
    client,
    options,
    address,
  }: CdtTokenReactQuery<TData> & { address: string }) {
    return useQuery<VotingPowerResponse, Error, TData>(
      ["cdtTokenVotingPower", client.contractAddress, address],
      () => client.votingPower({ address }),
      {
        ...options,
        enabled: !!client && (options?.enabled !== false || options?.enabled === undefined),
      }
    );
  }