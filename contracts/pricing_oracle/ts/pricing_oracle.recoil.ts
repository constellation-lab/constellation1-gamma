import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
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

type QueryClientParams = {
  contractAddress: string;
};

export const pricingOracleQueryClient = selectorFamily<PricingOracleQueryClient, QueryClientParams>({
  key: "pricingOracleQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new PricingOracleQueryClient(client, contractAddress);
  },
});

export const pricingOracleConfigSelector = selectorFamily<Config, QueryClientParams>({
  key: "pricingOracleConfig",
  get: (queryClientParams) => ({ get }) => {
    const client = get(pricingOracleQueryClient(queryClientParams));
    return client.config();
  },
});

export const pricingOracleCalculateOptionPriceSelector = selectorFamily
  CalculateOptionPriceResponse,
  QueryClientParams & {
    option_id: number;
    collateral: Uint128;
    counter_offer: Uint128;
    expiration: number;
  }
>({
  key: "pricingOracleCalculateOptionPrice",
  get: ({ option_id, collateral, counter_offer, expiration, ...queryClientParams }) => ({ get }) => {
    const client = get(pricingOracleQueryClient(queryClientParams));
    return client.calculateOptionPrice({
      option_id,
      collateral,
      counter_offer,
      expiration,
    });
  },
});

export const pricingOracleGetOptionPriceSelector = selectorFamily<OptionPrice, QueryClientParams & { option_id: number }>(
  {
    key: "pricingOracleGetOptionPrice",
    get: ({ option_id, ...queryClientParams }) => ({ get }) => {
      const client = get(pricingOracleQueryClient(queryClientParams));
      return client.getOptionPrice({ option_id });
    },
  }
);

export const pricingOracleGetPoolInfoSelector = selectorFamily<PoolInfo, QueryClientParams>({
    key: "pricingOracleGetPoolInfo",
    get: (queryClientParams) => ({ get }) => {
      const client = get(pricingOracleQueryClient(queryClientParams));
      return client.getPoolInfo();
    },
  });