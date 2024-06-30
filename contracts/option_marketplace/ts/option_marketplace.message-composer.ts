import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { ExecuteMsg, Uint128, Decimal } from "./option_marketplace.types";

export interface OptionMarketplaceMessageComposer {
  sender: string;
  contractAddress: string;
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
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  buyOption: (
    {
      amount,
      option_id,
    }: {
      amount: Uint128;
      option_id: number;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  executeOption: (
    {
      option_id,
    }: {
      option_id: number;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
}

export class OptionMarketplaceMessageComposer implements OptionMarketplaceMessageComposer {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.listOption = this.listOption.bind(this);
    this.buyOption = this.buyOption.bind(this);
    this.executeOption = this.executeOption.bind(this);
  }

  listOption = (
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
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            list_option: {
              min_trade_amount,
              option_id,
              price,
              slippage_tolerance,
            },
          })
        ),
        funds,
      }),
    };
  };

  buyOption = (
    {
      amount,
      option_id,
    }: {
      amount: Uint128;
      option_id: number;
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
            buy_option: { amount, option_id },
          })
        ),
        funds,
      }),
    };
  };

  executeOption = (
    {
      option_id,
    }: {
      option_id: number;
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
            execute_option: { option_id },
          })
        ),
        funds,
      }),
    };
  };
}