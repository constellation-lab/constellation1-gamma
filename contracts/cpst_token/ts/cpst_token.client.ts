import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import {
  InstantiateMsg,
  ExecuteMsg,
  QueryMsg,
  Uint128,
  Binary,
  Expiration,
  Timestamp,
  Uint64,
} from "./cpst_token.types";

export interface CpstTokenReadOnlyInterface {
  contractAddress: string;
  balance: ({ address }: { address: string }) => Promise<Uint128>;
  tokenInfo: () => Promise<unknown>;
  allowance: ({ owner, spender }: { owner: string; spender: string }) => Promise<Uint128>;
  allAllowances: ({
    owner,
    startAfter,
    limit,
  }: {
    owner: string;
    startAfter?: string;
    limit?: number;
  }) => Promise<unknown>;
  allAccounts: ({ startAfter, limit }: { startAfter?: string; limit?: number }) => Promise<unknown>;
  minter: () => Promise<unknown>;
  marketingInfo: () => Promise<unknown>;
  downloadLogo: () => Promise<unknown>;
}

export class CpstTokenQueryClient implements CpstTokenReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.balance = this.balance.bind(this);
    this.tokenInfo = this.tokenInfo.bind(this);
    this.allowance = this.allowance.bind(this);
    this.allAllowances = this.allAllowances.bind(this);
    this.allAccounts = this.allAccounts.bind(this);
    this.minter = this.minter.bind(this);
    this.marketingInfo = this.marketingInfo.bind(this);
    this.downloadLogo = this.downloadLogo.bind(this);
  }

  balance = async ({ address }: { address: string }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      balance: { address },
    });
  };

  tokenInfo = async (): Promise<unknown> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_info: {},
    });
  };

  allowance = async ({ owner, spender }: { owner: string; spender: string }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      allowance: { owner, spender },
    });
  };

  allAllowances = async ({
    owner,
    startAfter,
    limit,
  }: {
    owner: string;
    startAfter?: string;
    limit?: number;
  }): Promise<unknown> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_allowances: { owner, start_after: startAfter, limit },
    });
  };

  allAccounts = async ({ startAfter, limit }: { startAfter?: string; limit?: number }): Promise<unknown> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_accounts: { start_after: startAfter, limit },
    });
  };

  minter = async (): Promise<unknown> => {
    return this.client.queryContractSmart(this.contractAddress, {
      minter: {},
    });
  };

  marketingInfo = async (): Promise<unknown> => {
    return this.client.queryContractSmart(this.contractAddress, {
      marketing_info: {},
    });
  };

  downloadLogo = async (): Promise<unknown> => {
    return this.client.queryContractSmart(this.contractAddress, {
      download_logo: {},
    });
  };
}

export interface CpstTokenInterface extends CpstTokenReadOnlyInterface {
  contractAddress: string;
  sender: string;

  transfer: (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  burn: (
    {
      amount,
    }: {
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  send: (
    {
      contract,
      amount,
      msg,
    }: {
      contract: string;
      amount: Uint128;
      msg: Binary;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  mint: (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  increaseAllowance: (
    {
      spender,
      amount,
      expires,
    }: {
      spender: string;
      amount: Uint128;
      expires?: Expiration;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  decreaseAllowance: (
    {
      spender,
      amount,
      expires,
    }: {
      spender: string;
      amount: Uint128;
      expires?: Expiration;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  transferFrom: (
    {
      owner,
      recipient,
      amount,
    }: {
      owner: string;
      recipient: string;
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  burnFrom: (
    {
      owner,
      amount,
    }: {
      owner: string;
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  sendFrom: (
    {
      owner,
      contract,
      amount,
      msg,
    }: {
      owner: string;
      contract: string;
      amount: Uint128;
      msg: Binary;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;

  stake: (
    {
      amount,
    }: {
      amount: Uint128;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}

export class CpstTokenClient extends CpstTokenQueryClient implements CpstTokenInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.transfer = this.transfer.bind(this);
    this.burn = this.burn.bind(this);
    this.send = this.send.bind(this);
    this.mint = this.mint.bind(this);
    this.increaseAllowance = this.increaseAllowance.bind(this);
    this.decreaseAllowance = this.decreaseAllowance.bind(this);
    this.transferFrom = this.transferFrom.bind(this);
    this.burnFrom = this.burnFrom.bind(this);
    this.sendFrom = this.sendFrom.bind(this);
    this.stake = this.stake.bind(this);
  }

  transfer = async (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        transfer: { recipient, amount },
      },
      fee,
      memo,
      funds
    );
  };

  burn = async (
    {
      amount,
    }: {
      amount: Uint128;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        burn: { amount },
      },
      fee,
      memo,
      funds
    );
  };

  send = async (
    {
      contract,
      amount,
      msg,
    }: {
      contract: string;
      amount: Uint128;
      msg: Binary;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        send: { contract, amount, msg },
      },
      fee,
      memo,
      funds
    );
  };

  mint = async (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        mint: { recipient, amount },
      },
      fee,
      memo,
      funds
    );
  };

  increaseAllowance = async (
    {
      spender,
      amount,
      expires,
    }: {
      spender: string;
      amount: Uint128;
      expires?: Expiration;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        increase_allowance: { spender, amount, expires },
      },
      fee,
      memo,
      funds
    );
  };

  decreaseAllowance = async (
    {
      spender,
      amount,
      expires,
    }: {
      spender: string;
      amount: Uint128;
      expires?: Expiration;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        decrease_allowance: { spender, amount, expires },
      },
      fee,
      memo,
      funds
    );
  };

  transferFrom = async (
    {
      owner,
      recipient,
      amount,
    }: {
      owner: string;
      recipient: string;
      amount: Uint128;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        transfer_from: { owner, recipient, amount },
      },
      fee,
      memo,
      funds
    );
  };

  burnFrom = async (
    {
      owner,
      amount,
    }: {
      owner: string;
      amount: Uint128
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        burn_from: { owner, amount },
      },
      fee,
      memo,
      funds
    );
  };

  sendFrom = async (
    {
      owner,
      contract,
      amount,
      msg,
    }: {
      owner: string;
      contract: string;
      amount: Uint128;
      msg: Binary;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        send_from: { owner, contract, amount, msg },
      },
      fee,
      memo,
      funds
    );
  };

  stake = async (
    {
      amount,
    }: {
      amount: Uint128;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        stake: { amount },
      },
      fee,
      memo,
      funds
    );
  };
}