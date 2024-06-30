import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { InstantiateMsg, ExecuteMsg, QueryMsg, Asset } from "./liquidity_pool.types";

export interface LiquidityPoolMessageComposer {
  sender: string;
  contractAddress: string;
  deposit: (
    {
      assets,
    }: {
      assets: Asset[];
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  withdraw: (
    {
      lpTokens,
    }: {
      lpTokens: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
}

export class LiquidityPoolMessageComposer implements LiquidityPoolMessageComposer {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.deposit = this.deposit.bind(this);
    this.withdraw = this.withdraw.bind(this);
  }

  deposit = (
    {
      assets,
    }: {
      assets: Asset[];
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            deposit: { assets },
          })
        ),
        funds,
      }),
    };
  };

  withdraw = (
    {
      lpTokens,
    }: {
      lpTokens: string;
    },
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            withdraw: { lp_tokens: lpTokens },
          })
        ),
        funds,
      }),
    };
  };
}