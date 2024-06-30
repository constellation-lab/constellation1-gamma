import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import {
  ExecuteMsg,
  QueryMsg,
  Config,
  PositionLimit,
  CircuitBreaker,
  PoolInfo,
  OptionPosition,
  Uint128,
  Decimal,
} from "./risk_management.types";
import { RiskManagementQueryClient } from "./risk_management.client";

type QueryClientParams = {
  contractAddress: string;
};

export const riskManagementQueryClient = selectorFamily<RiskManagementQueryClient, QueryClientParams>({
  key: "riskManagementQueryClient",
  get: ({ contractAddress }) => ({ get }) => {
    const client = get(cosmWasmClient);
    return new RiskManagementQueryClient(client, contractAddress);
  },
});

export const riskManagementConfigSelector = selectorFamily<Config, QueryClientParams>({
  key: "riskManagementConfig",
  get: (queryClientParams) => ({ get }) => {
    const client = get(riskManagementQueryClient(queryClientParams));
    return client.config();
  },
});

export const riskManagementPositionLimitSelector = selectorFamily
  PositionLimit,
  QueryClientParams & { option_pair: string }
>({
  key: "riskManagementPositionLimit",
  get: ({ option_pair, ...queryClientParams }) => ({ get }) => {
    const client = get(riskManagementQueryClient(queryClientParams));
    return client.positionLimit({ option_pair });
  },
});

export const riskManagementCircuitBreakerSelector = selectorFamily
  CircuitBreaker,
  QueryClientParams & { option_pair: string }
>({
  key: "riskManagementCircuitBreaker",
  get: ({ option_pair, ...queryClientParams }) => ({ get }) => {
    const client = get(riskManagementQueryClient(queryClientParams));
    return client.circuitBreaker({ option_pair });
  },
});

export const riskManagementGetPoolInfoSelector = selectorFamily<PoolInfo, QueryClientParams>({
  key: "riskManagementGetPoolInfo",
  get: (queryClientParams) => ({ get }) => {
    const client = get(riskManagementQueryClient(queryClientParams));
    return client.getPoolInfo();
  },
});

export const riskManagementGetOptionPositionsSelector = selectorFamily<OptionPosition[], QueryClientParams>({
  key: "riskManagementGetOptionPositions",
  get: (queryClientParams) => ({ get }) => {
    const client = get(riskManagementQueryClient(queryClientParams));
    return client.getOptionPositions();
  },
});

export const riskManagementGetPriceSelector = selectorFamily<Decimal, QueryClientParams & { option_pair: string }>({
  key: "riskManagementGetPrice",
  get: ({ option_pair, ...queryClientParams }) => ({ get }) => {
    const client = get(riskManagementQueryClient(queryClientParams));
    return client.getPrice({ option_pair });
  },
});