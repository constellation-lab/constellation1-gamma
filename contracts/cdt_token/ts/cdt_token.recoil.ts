import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import {
  ExecuteMsg,
  QueryMsg,
  VotingPowerResponse,
  Uint128,
  Binary,
} from "./cdt_token.types";
import { CdtTokenQueryClient } from "./cdt_token.client";

type QueryClientParams = {
  contractAddress: string;
};

export const cdtTokenQueryClient = selectorFamily<CdtTokenQueryClient, QueryClientParams>({
  key: "cdtTokenQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new CdtTokenQueryClient(client, contractAddress);
  },
});

export const cdtTokenBalanceSelector = selectorFamily<Uint128, QueryClientParams & { address: string }>({
  key: "cdtTokenBalance",
  get: ({ address, ...queryClientParams }) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.balance({ address });
  },
});

export const cdtTokenTokenInfoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cdtTokenTokenInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.tokenInfo();
  },
});

export const cdtTokenAllowanceSelector = selectorFamily
  Uint128,
  QueryClientParams & { owner: string; spender: string }
>({
  key: "cdtTokenAllowance",
  get: ({ owner, spender, ...queryClientParams }) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.allowance({ owner, spender });
  },
});

export const cdtTokenAllAllowancesSelector = selectorFamily
  unknown,
  QueryClientParams & { owner: string; startAfter?: string; limit?: number }
>({
  key: "cdtTokenAllAllowances",
  get: ({ owner, startAfter, limit, ...queryClientParams }) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.allAllowances({ owner, startAfter, limit });
  },
});

export const cdtTokenAllAccountsSelector = selectorFamily
  unknown,
  QueryClientParams & { startAfter?: string; limit?: number }
>({
  key: "cdtTokenAllAccounts",
  get: ({ startAfter, limit, ...queryClientParams }) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.allAccounts({ startAfter, limit });
  },
});

export const cdtTokenMinterSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cdtTokenMinter",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.minter();
  },
});

export const cdtTokenMarketingInfoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cdtTokenMarketingInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.marketingInfo();
  },
});

export const cdtTokenDownloadLogoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cdtTokenDownloadLogo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cdtTokenQueryClient(queryClientParams));
    return client.downloadLogo();
  },
});

export const cdtTokenVotingPowerSelector = selectorFamily<VotingPowerResponse, QueryClientParams & { address: string }>(
  {
    key: "cdtTokenVotingPower",
    get: ({ address, ...queryClientParams }) => ({ get }) => {
      const client = get(cdtTokenQueryClient(queryClientParams));
      return client.votingPower({ address });
    },
  }
);