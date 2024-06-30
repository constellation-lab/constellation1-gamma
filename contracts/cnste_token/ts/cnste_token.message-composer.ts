import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { ExecuteMsg, Uint128, Binary } from "./cnste_token.types";

export interface CnsteTokenMessageComposer {
  sender: string;
  contractAddress: string;

  transfer: (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  burn: (
    {
      amount,
    }: {
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  mint: (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

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
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  increaseAllowance: (
    {
      spender,
      amount,
    }: {
      spender: string;
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  decreaseAllowance: (
    {
      spender,
      amount,
    }: {
      spender: string;
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

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
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  burnFrom: (
    {
      owner,
      amount,
    }: {
      owner: string;
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

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
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;

  stake: (
    {
      amount,
    }: {
      amount: Uint128;
    },
    funds?: Coin[]
  ) => MsgExecuteContractEncodeObject;
}

export class CnsteTokenMessageComposer implements CnsteTokenMessageComposer {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.transfer = this.transfer.bind(this);
    this.burn = this.burn.bind(this);
    this.mint = this.mint.bind(this);
    this.send = this.send.bind(this);
    this.increaseAllowance = this.increaseAllowance.bind(this);
    this.decreaseAllowance = this.decreaseAllowance.bind(this);
    this.transferFrom = this.transferFrom.bind(this);
    this.burnFrom = this.burnFrom.bind(this);
    this.sendFrom = this.sendFrom.bind(this);
    this.stake = this.stake.bind(this);
  }

  transfer = (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
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
            transfer: { recipient, amount },
          })
        ),
        funds,
      }),
    };
  };

  burn = (
    {
      amount,
    }: {
      amount: Uint128;
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
            burn: { amount },
          })
        ),
        funds,
      }),
    };
  };

  mint = (
    {
      recipient,
      amount,
    }: {
      recipient: string;
      amount: Uint128;
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
            mint: { recipient, amount },
          })
        ),
        funds,
      }),
    };
  };

  send = (
    {
      contract,
      amount,
      msg,
    }: {
      contract: string;
      amount: Uint128;
      msg: Binary;
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
            send: { contract, amount, msg },
          })
        ),
        funds,
      }),
    };
  };

  increaseAllowance = (
    {
      spender,
      amount,
    }: {
      spender: string;
      amount: Uint128;
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
            increase_allowance: { spender, amount },
          })
        ),
        funds,
      }),
    };
  };

  decreaseAllowance = (
    {
      spender,
      amount,
    }: {
      spender: string;
      amount: Uint128;
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
            decrease_allowance: { spender, amount },
          })
        ),
        funds,
      }),
    };
  };

  transferFrom = (
    {
      owner,
      recipient,
      amount,
    }: {
      owner: string;
      recipient: string;
      amount: Uint128;
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
            transfer_from: { owner, recipient, amount },
          })
        ),
        funds,
      }),
    };
  };

  burnFrom = (
    {
      owner,
      amount,
    }: {
      owner: string;
      amount: Uint128;
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
            burn_from: { owner, amount },
          })
        ),
        funds,
      }),
    };
  };

  sendFrom = (
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
    funds?: Coin[]
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            send_from: { owner, contract, amount, msg },
          })
        ),
        funds,
      }),
    };
  };

  stake = (
    {
      amount,
    }: {
      amount: Uint128;
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
            stake: { amount },
          })
        ),
        funds,
      }),
    };
  };
}