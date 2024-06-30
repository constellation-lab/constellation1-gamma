import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import {
  InstantiateMsg,
  ExecuteMsg,
  QueryMsg,
  Config,
  CalculateOptionPriceResponse,
  OptionPrice,
  PoolInfo,
  Uint128,
} from "./pricing_oracle.types";

export interface PricingOracleReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  calculateOptionPrice: ({
    option_id,
    collateral,
    counter_offer,
    expiration,
  }: {
    option_id: number;
    collateral: Uint128;
    counter_offer: Uint128;
    expiration: number;
  }) => Promise<CalculateOptionPriceResponse>;
  getOptionPrice: ({ option_id }: { option_id: number }) => Promise<OptionPrice>;
  getPoolInfo: () => Promise<PoolInfo>;
}

export class PricingOracleQueryClient implements PricingOracleReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.calculateOptionPrice = this.calculateOptionPrice.bind(this);
    this.getOptionPrice = this.getOptionPrice.bind(this);
    this.getPoolInfo = this.getPoolInfo.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {},
    });
  };

  calculateOptionPrice = async ({
    option_id,
    collateral,
    counter_offer,
    expiration,
  }: {
    option_id: number;
    collateral: Uint128;
    counter_offer: Uint128;
    expiration: number;
  }): Promise<CalculateOptionPriceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      calculate_option_price: {
        option_id,
        collateral,
        counter_offer,
        expiration,
      },
    });
  };

  getOptionPrice = async ({ option_id }: { option_id: number }): Promise<OptionPrice> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_option_price: { option_id },
    });
  };

  getPoolInfo = async (): Promise<PoolInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_pool_info: {},
    });
  };
}

export interface PricingOracleInterface extends PricingOracleReadOnlyInterface {
  contractAddress: string;
  sender: string;
  updateConfig: (
    {
      admin,
      liquidity_pool_contract,
      constella_option_contract,
    }: {
      admin?: string;
      liquidity_pool_contract?: string;
      constella_option_contract?: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  saveOptionPrice: (
    {
      option_id,
      price,
    }: {
      option_id: number;
      price: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}

export class PricingOracleClient extends PricingOracleQueryClient implements PricingOracleInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateConfig = this.updateConfig.bind(this);
    this.saveOptionPrice = this.saveOptionPrice.bind(this);
  }

  updateConfig = async (
    {
      admin,
      liquidity_pool_contract,
      constella_option_contract,
    }: {
      admin?: string;
      liquidity_pool_contract?: string;
      constella_option_contract?: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_config: {
          admin,
          liquidity_pool_contract,
          constella_option_contract,
        },
      },
      fee,
      memo,
      funds
    );
  };

  saveOptionPrice = async (
    {
      option_id,
      price,
    }: {
      option_id: number;
      price: Uint128;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        save_option_price: { option_id, price },
      },
      fee,
      memo,
      funds
    );
  };
}