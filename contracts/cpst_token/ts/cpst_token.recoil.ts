import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
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

type QueryClientParams = {
  contractAddress: string;
};

export const cpstTokenQueryClient = selectorFamily<CpstTokenQueryClient, QueryClientParams>({
  key: "cpstTokenQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new CpstTokenQueryClient(client, contractAddress);
  },
});

export const cpstTokenBalanceSelector = selectorFamily<Uint128, QueryClientParams & { address: string }>({
  key: "cpstTokenBalance",
  get: ({ address, ...queryClientParams }) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.balance({ address });
  },
});

export const cpstTokenTokenInfoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cpstTokenTokenInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.tokenInfo();
  },
});

export const cpstTokenAllowanceSelector = selectorFamily
  Uint128,
  QueryClientParams & { owner: string; spender: string }
>({
  key: "cpstTokenAllowance",
  get: ({ owner, spender, ...queryClientParams }) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.allowance({ owner, spender });
  },
});

export const cpstTokenAllAllowancesSelector = selectorFamily
  unknown,
  QueryClientParams & { owner: string; startAfter?: string; limit?: number }
>({
  key: "cpstTokenAllAllowances",
  get: ({ owner, startAfter, limit, ...queryClientParams }) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.allAllowances({ owner, startAfter, limit });
  },
});

export const cpstTokenAllAccountsSelector = selectorFamily
  unknown,
  QueryClientParams & { startAfter?: string; limit?: number }
>({
  key: "cpstTokenAllAccounts",
  get: ({ startAfter, limit, ...queryClientParams }) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.allAccounts({ startAfter, limit });
  },
});

export const cpstTokenMinterSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cpstTokenMinter",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.minter();
  },
});

export const cpstTokenMarketingInfoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cpstTokenMarketingInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.marketingInfo();
  },
});

export const cpstTokenDownloadLogoSelector = selectorFamily<unknown, QueryClientParams>({
  key: "cpstTokenDownloadLogo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(cpstTokenQueryClient(queryClientParams));
    return client.downloadLogo();
  },
});