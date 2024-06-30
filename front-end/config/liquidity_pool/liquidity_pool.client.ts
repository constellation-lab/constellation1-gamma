import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee, Coin } from "@cosmjs/amino";
import {
  Asset,
  InstantiateMsg,
  ExecuteMsg,
  QueryMsg,
  LiquidityPool,
  LiquidityProviderInfo,
} from "./liquidity_pool.types";

export interface LiquidityPoolReadOnlyInterface {
  contractAddress: string;
  getPool: () => Promise<LiquidityPool>;
  getLiquidityProviderInfo: ({ address }: { address: string }) => Promise<LiquidityProviderInfo>;
}

export class LiquidityPoolQueryClient implements LiquidityPoolReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getPool = this.getPool.bind(this);
    this.getLiquidityProviderInfo = this.getLiquidityProviderInfo.bind(this);
  }

  getPool = async (): Promise<LiquidityPool> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_pool: {},
    });
  };

  getLiquidityProviderInfo = async ({
    address,
  }: {
    address: string;
  }): Promise<LiquidityProviderInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_liquidity_provider_info: { address },
    });
  };
}

export interface LiquidityPoolInterface extends LiquidityPoolReadOnlyInterface {
  contractAddress: string;
  sender: string;
  deposit: (
    {
      assets,
    }: {
      assets: Asset[];
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  withdraw: (
    {
      lpTokens,
    }: {
      lpTokens: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}

export class LiquidityPoolClient
  extends LiquidityPoolQueryClient
  implements LiquidityPoolInterface
{
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(
    client: SigningCosmWasmClient,
    sender: string,
    contractAddress: string
  ) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.deposit = this.deposit.bind(this);
    this.withdraw = this.withdraw.bind(this);
  }

  deposit = async (
    {
      assets,
    }: {
      assets: Asset[];
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        deposit: { assets },
      },
      fee,
      memo,
      funds
    );
  };

  withdraw = async (
    {
      lpTokens,
    }: {
      lpTokens: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        withdraw: { lp_tokens: lpTokens },
      },
      fee,
      memo,
      funds
    );
  };

  provideLiquidity = async (
    { assets }: { assets: Asset[] },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        provide_liquidity: { assets },
      },
      fee,
      memo,
      funds
    );
  };



  async executeSwap(
    senderAddress: string,
    params: SwapParams,
    fee: StdFee | "auto" | number,
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> {
    return this.client.execute(
      senderAddress,
      this.contractAddress,
      {
        swap: params
      },
      fee,
      memo,
      funds
    );
  }
}

export interface SwapParams {
  offer_asset: Asset;
  //ask_asset_info: AssetInfo;
  // Add any other necessary parameters
}