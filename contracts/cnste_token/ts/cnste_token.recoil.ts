import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import {
  ExecuteMsg,
  QueryMsg,
  StakeInfo,
  Uint128,
  Binary,
} from "./cnste_token.types";
import { CnsteTokenQueryClient } from "./cnste_token.client";

type QueryClientParams = {
  contractAddress: string;
};

export const cnsteTokenQueryClient = selectorFamily<CnsteTokenQueryClient, QueryClientParams>({
  key: "cnsteTokenQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new CnsteTokenQueryClient(client, contractAddress);
  },
});

export const cnsteTokenBalanceSelector = selectorFamily<Uint128, QueryClientParams & { address: string }>({
  key: "cnsteTokenBalance",
  get: ({ address, ...queryClientParams }) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.balance({ address });
  },
});

export const cnsteTokenTokenInfoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cnsteTokenTokenInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.tokenInfo();
  },
});

export const cnsteTokenAllowanceSelector = selectorFamily
  Uint128,
  QueryClientParams & { owner: string; spender: string }
>({
  key: "cnsteTokenAllowance",
  get: ({ owner, spender, ...queryClientParams }) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.allowance({ owner, spender });
  },
});

export const cnsteTokenAllAllowancesSelector = selectorFamily
  unknown,
  QueryClientParams & { owner: string; startAfter?: string; limit?: number }
>({
  key: "cnsteTokenAllAllowances",
  get: ({ owner, startAfter, limit, ...queryClientParams }) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.allAllowances({ owner, startAfter, limit });
  },
});

export const cnsteTokenAllAccountsSelector = selectorFamily
  unknown,
  QueryClientParams & { startAfter?: string; limit?: number }
>({
  key: "cnsteTokenAllAccounts",
  get: ({ startAfter, limit, ...queryClientParams }) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.allAccounts({ startAfter, limit });
  },
});

export const cnsteTokenMinterSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cnsteTokenMinter",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.minter();
  },
});

export const cnsteTokenMarketingInfoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cnsteTokenMarketingInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.marketingInfo();
  },
});

export const cnsteTokenDownloadLogoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cnsteTokenDownloadLogo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.downloadLogo();
  },
});

export const cnsteTokenStakeInfoSelector = selectorFamily<StakeInfo, QueryClientParams & { address: string }>({
  key: "cnsteTokenStakeInfo",
  get: ({ address, ...queryClientParams }) => ({ get }) => {
    const client = get(cnsteTokenQueryClient(queryClientParams));
    return client.stakeInfo({ address });
  },
});