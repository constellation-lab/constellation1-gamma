import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { ExecuteMsg, Uint128, Decimal } from "./risk_management.types";

export interface RiskManagementMessageComposer {
  sender: string;
  contractAddress: string;
  setPositionLimit: (
    {
      max_position,
      option_pair,
    }: {
      max_position: Uint128;
      option_pair: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
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
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  executeRiskMitigationStrategy: (funds?: Coin[]) => MsgExecuteContractEncodeObject;
  adjustPricing: (
    {
      adjustment_factor,
      option_pair,
    }: {
      adjustment_factor: Decimal;
      option_pair: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  closePosition: (
    {
      amount,
      option_pair,
    }: {
      amount: Uint128;
      option_pair: string;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
  adjustParameters: (
    {
      volatility_multiplier,
    }: {
      volatility_multiplier: Decimal;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
}

export class RiskManagementMessageComposer implements RiskManagementMessageComposer {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.setPositionLimit = this.setPositionLimit.bind(this);
    this.setCircuitBreaker = this.setCircuitBreaker.bind(this);
    this.executeRiskMitigationStrategy = this.executeRiskMitigationStrategy.bind(this);
    this.adjustPricing = this.adjustPricing.bind(this);
    this.closePosition = this.closePosition.bind(this);
    this.adjustParameters = this.adjustParameters.bind(this);
  }

  setPositionLimit = (
    {
      max_position,
      option_pair,
    }: {
      max_position: Uint128;
      option_pair: string;
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
            set_position_limit: {
              max_position,
              option_pair,
            },
          })
        ),
        funds,
      }),
    };
  };

  setCircuitBreaker = (
    {
      option_pair,
      price_threshold,
      triggered,
    }: {
      option_pair: string;
      price_threshold: Decimal;
      triggered: boolean;
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
            set_circuit_breaker: {
              option_pair,
              price_threshold,
              triggered,
            },
          })
        ),
        funds,
      }),
    };
  };

  executeRiskMitigationStrategy = (funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            execute_risk_mitigation_strategy: {},
          })
        ),
        funds,
      }),
    };
  };

  adjustPricing = (
    {
      adjustment_factor,
      option_pair,
    }: {
      adjustment_factor: Decimal;
      option_pair: string;
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
            adjust_pricing: {
              adjustment_factor,
              option_pair,
            },
          })
        ),
        funds,
      }),
    };
  };

  closePosition = (
    {
      amount,
      option_pair,
    }: {
      amount: Uint128;
      option_pair: string;
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
            close_position: {
              amount,
              option_pair,
            },
          })
        ),
        funds,
      }),
    };
  };

  adjustParameters = (
    {
      volatility_multiplier,
    }: {
      volatility_multiplier: Decimal;
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
            adjust_parameters: {
              volatility_multiplier,
            },
          })
        ),
        funds,
      }),
    };
  };
}