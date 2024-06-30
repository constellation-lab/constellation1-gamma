import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { ExecuteMsg, Uint128 } from "./pricing_oracle.types";

export interface PricingOracleMessageComposer {
  sender: string;
  contractAddress: string;
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
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  saveOptionPrice: (
    {
      option_id,
      price,
    }: {
      option_id: number;
      price: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
}

export class PricingOracleMessageComposer implements PricingOracleMessageComposer {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateConfig = this.updateConfig.bind(this);
    this.saveOptionPrice = this.saveOptionPrice.bind(this);
  }

  updateConfig = (
    {
      admin,
      liquidity_pool_contract,
      constella_option_contract,
    }: {
      admin?: string;
      liquidity_pool_contract?: string;
      constella_option_contract?: string;
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
            update_config: {
              admin,
              liquidity_pool_contract,
              constella_option_contract,
            },
          })
        ),
        funds,
      }),
    };
  };

  saveOptionPrice = (
    {
      option_id,
      price,
    }: {
      option_id: number;
      price: Uint128;
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
            save_option_price: { option_id, price },
          })
        ),
        funds,
      }),
    };
  };
}