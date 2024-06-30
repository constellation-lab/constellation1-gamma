import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import { ExecuteMsg, QueryMsg, Option, Uint128, Decimal } from "./option_marketplace.types";
import { OptionMarketplaceQueryClient } from "./option_marketplace.client";

type QueryClientParams = {
  contractAddress: string;
};

export const optionMarketplaceQueryClient = selectorFamily<OptionMarketplaceQueryClient, QueryClientParams>({
  key: "optionMarketplaceQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new OptionMarketplaceQueryClient(client, contractAddress);
  },
});

export const optionMarketplaceGetOptionSelector = selectorFamily<Option, QueryClientParams & { option_id: number }>({
  key: "optionMarketplaceGetOption",
  get: ({ option_id, ...queryClientParams }) => ({ get }) => {
    const client = get(optionMarketplaceQueryClient(queryClientParams));
    return client.getOption({ option_id });
  },
});

export const optionMarketplaceListOptionsSelector = selectorFamily<Option[], QueryClientParams>({
  key: "optionMarketplaceListOptions",
  get: (queryClientParams) => ({ get }) => {
    const client = get(optionMarketplaceQueryClient(queryClientParams));
    return client.listOptions();
  },
});

export const optionMarketplaceGetOptionPriceSelector = selectorFamily
  Uint128,
  QueryClientParams & { option_id: number; slippage_tolerance: Decimal }
>({
  key: "optionMarketplaceGetOptionPrice",
  get: ({ option_id, slippage_tolerance, ...queryClientParams }) => ({ get }) => {
    const client = get(optionMarketplaceQueryClient(queryClientParams));
    return client.getOptionPrice({ option_id, slippage_tolerance });
  },
});