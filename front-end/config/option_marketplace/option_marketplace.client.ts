import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee, Coin } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, QueryMsg, Option, Uint128, Decimal } from "./option_marketplace.types";

export interface OptionMarketplaceReadOnlyInterface {
  contractAddress: string;
  getOption: ({ option_id }: { option_id: number }) => Promise<Option>;
  listOptions: () => Promise<Option[]>;
  getOptionPrice: ({
    option_id,
    slippage_tolerance,
  }: {
    option_id: number;
    slippage_tolerance: Decimal;
  }) => Promise<Uint128>;
}

export class OptionMarketplaceQueryClient implements OptionMarketplaceReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getOption = this.getOption.bind(this);
    this.listOptions = this.listOptions.bind(this);
    this.getOptionPrice = this.getOptionPrice.bind(this);
  }

  getOption = async ({ option_id }: { option_id: number }): Promise<Option> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_option: { option_id },
    });
  };

  listOptions = async (): Promise<Option[]> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_options: {},
    });
  };

  getOptionPrice = async ({
    option_id,
    slippage_tolerance,
  }: {
    option_id: number;
    slippage_tolerance: Decimal;
  }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_option_price: { option_id, slippage_tolerance },
    });
  };
}

export interface OptionMarketplaceInterface extends OptionMarketplaceReadOnlyInterface {
  contractAddress: string;
  sender: string;
  listOption: (
    {
      min_trade_amount,
      option_id,
      price,
      slippage_tolerance,
    }: {
      min_trade_amount: Uint128;
      option_id: number;
      price: Uint128;
      slippage_tolerance: Decimal;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  buyOption: (
    {
      amount,
      option_id,
    }: {
      amount: Uint128;
      option_id: number;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  executeOption: (
    {
      option_id,
    }: {
      option_id: number;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}

export class OptionMarketplaceClient extends OptionMarketplaceQueryClient implements OptionMarketplaceInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.listOption = this.listOption.bind(this);
    this.buyOption = this.buyOption.bind(this);
    this.executeOption = this.executeOption.bind(this);
  }

  listOption = async (
    {
      min_trade_amount,
      option_id,
      price,
      slippage_tolerance,
    }: {
      min_trade_amount: Uint128;
      option_id: number;
      price: Uint128;
      slippage_tolerance: Decimal;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        list_option: {
          min_trade_amount,
          option_id,
          price,
          slippage_tolerance,
        },
      },
      fee,
      memo,
      funds
    );
  };

  buyOption = async (
    {
      amount,
      option_id,
    }: {
      amount: Uint128;
      option_id: number;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        buy_option: { amount, option_id },
      },
      fee,
      memo,
      funds
    );
  };

  executeOption = async (
    {
      option_id,
    }: {
      option_id: number;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        execute_option: { option_id },
      },
      fee,
      memo,
      funds
    );
  };
}