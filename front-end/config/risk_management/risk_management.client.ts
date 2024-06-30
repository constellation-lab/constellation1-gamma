import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee, Coin } from "@cosmjs/amino";
import {
  InstantiateMsg,
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

export interface RiskManagementReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  positionLimit: ({ option_pair }: { option_pair: string }) => Promise<PositionLimit>;
  circuitBreaker: ({ option_pair }: { option_pair: string }) => Promise<CircuitBreaker>;
  getPoolInfo: () => Promise<PoolInfo>;
  getOptionPositions: () => Promise<OptionPosition[]>;
  getPrice: ({ option_pair }: { option_pair: string }) => Promise<Decimal>;
}

export class RiskManagementQueryClient implements RiskManagementReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.positionLimit = this.positionLimit.bind(this);
    this.circuitBreaker = this.circuitBreaker.bind(this);
    this.getPoolInfo = this.getPoolInfo.bind(this);
    this.getOptionPositions = this.getOptionPositions.bind(this);
    this.getPrice = this.getPrice.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {},
    });
  };

  positionLimit = async ({ option_pair }: { option_pair: string }): Promise<PositionLimit> => {
    return this.client.queryContractSmart(this.contractAddress, {
      position_limit: { option_pair },
    });
  };

  circuitBreaker = async ({ option_pair }: { option_pair: string }): Promise<CircuitBreaker> => {
    return this.client.queryContractSmart(this.contractAddress, {
      circuit_breaker: { option_pair },
    });
  };

  getPoolInfo = async (): Promise<PoolInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_pool_info: {},
    });
  };

  getOptionPositions = async (): Promise<OptionPosition[]> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_option_positions: {},
    });
  };

  getPrice = async ({ option_pair }: { option_pair: string }): Promise<Decimal> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_price: { option_pair },
    });
  };
}

export interface RiskManagementInterface extends RiskManagementReadOnlyInterface {
  contractAddress: string;
  sender: string;

  setPositionLimit: (
    {
      max_position,
      option_pair,
    }: {
      max_position: Uint128;
      option_pair: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  setCircuitBreaker: (
    {
      option_pair,
      price_threshold,
      triggered,
    }: {
      option_pair: string;
      price_threshold: Decimal;
      triggered: boolean;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  executeRiskMitigationStrategy: (
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  adjustPricing: (
    {
      adjustment_factor,
      option_pair,
    }: {
      adjustment_factor: Decimal;
      option_pair: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  closePosition: (
    {
      amount,
      option_pair,
    }: {
      amount: Uint128;
      option_pair: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  adjustParameters: (
    {
      volatility_multiplier,
    }: {
      volatility_multiplier: Decimal;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}

export class RiskManagementClient extends RiskManagementQueryClient implements RiskManagementInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.setPositionLimit = this.setPositionLimit.bind(this);
    this.setCircuitBreaker = this.setCircuitBreaker.bind(this);
    this.executeRiskMitigationStrategy = this.executeRiskMitigationStrategy.bind(this);
    this.adjustPricing = this.adjustPricing.bind(this);
    this.closePosition = this.closePosition.bind(this);
    this.adjustParameters = this.adjustParameters.bind(this);
  }

  setPositionLimit = async (
    {
      max_position,
      option_pair,
    }: {
      max_position: Uint128;
      option_pair: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        set_position_limit: {
          max_position,
          option_pair,
        },
      },
      fee,
      memo,
      funds
    );
  };

  setCircuitBreaker = async (
    {
      option_pair,
      price_threshold,
      triggered,
    }: {
      option_pair: string;
      price_threshold: Decimal;
      triggered: boolean;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        set_circuit_breaker: {
          option_pair,
          price_threshold,
          triggered,
        },
      },
      fee,
      memo,
      funds
    );
  };

  executeRiskMitigationStrategy = async (
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        execute_risk_mitigation_strategy: {},
      },
      fee,
      memo,
      funds
    );
  };

  adjustPricing = async (
    {
      adjustment_factor,
      option_pair,
    }: {
      adjustment_factor: Decimal;
      option_pair: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        adjust_pricing: {
          adjustment_factor,
          option_pair,
        },
      },
      fee,
      memo,
      funds
    );
  };

  closePosition = async (
    {
      amount,
      option_pair,
    }: {
      amount: Uint128;
      option_pair: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        close_position: {
          amount,
          option_pair,
        },
      },
      fee,
      memo,
      funds
    );
  };

  adjustParameters = async (
    {
      volatility_multiplier,
    }: {
      volatility_multiplier: Decimal;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        adjust_parameters: {
          volatility_multiplier,
        },
      },
      fee,
      memo,
      funds
    );
  };
}