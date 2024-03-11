/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.7.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, Uint128, QueryMsg, Addr, Timestamp, Uint64, ArrayOfTupleOfAddrAndBid, Bid, ContractParams, ListItemData, Coin, ArrayOfTupleOfUint64AndListItemData } from "./ContellaMarket.types";
export interface ContellaMarketReadOnlyInterface {
  contractAddress: string;
  contractParams: () => Promise<ContractParams>;
  listItems: () => Promise<ArrayOfTupleOfUint64AndListItemData>;
  listItemsPage: ({
    amount,
    key
  }: {
    amount: number;
    key: number;
  }) => Promise<ArrayOfTupleOfUint64AndListItemData>;
  getListItemsByid: ({
    id
  }: {
    id: number;
  }) => Promise<ListItemData>;
  ownerListItems: ({
    addr
  }: {
    addr: string;
  }) => Promise<ArrayOfTupleOfUint64AndListItemData>;
  ownerUnListItems: ({
    addr
  }: {
    addr: string;
  }) => Promise<ArrayOfTupleOfUint64AndListItemData>;
  bidList: ({
    id
  }: {
    id: number;
  }) => Promise<ArrayOfTupleOfAddrAndBid>;
}
export class ContellaMarketQueryClient implements ContellaMarketReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.contractParams = this.contractParams.bind(this);
    this.listItems = this.listItems.bind(this);
    this.listItemsPage = this.listItemsPage.bind(this);
    this.getListItemsByid = this.getListItemsByid.bind(this);
    this.ownerListItems = this.ownerListItems.bind(this);
    this.ownerUnListItems = this.ownerUnListItems.bind(this);
    this.bidList = this.bidList.bind(this);
  }

  contractParams = async (): Promise<ContractParams> => {
    return this.client.queryContractSmart(this.contractAddress, {
      contract_params: {}
    });
  };
  listItems = async (): Promise<ArrayOfTupleOfUint64AndListItemData> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_items: {}
    });
  };
  listItemsPage = async ({
    amount,
    key
  }: {
    amount: number;
    key: number;
  }): Promise<ArrayOfTupleOfUint64AndListItemData> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_items_page: {
        amount,
        key
      }
    });
  };
  getListItemsByid = async ({
    id
  }: {
    id: number;
  }): Promise<ListItemData> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_list_items_byid: {
        id
      }
    });
  };
  ownerListItems = async ({
    addr
  }: {
    addr: string;
  }): Promise<ArrayOfTupleOfUint64AndListItemData> => {
    return this.client.queryContractSmart(this.contractAddress, {
      owner_list_items: {
        addr
      }
    });
  };
  ownerUnListItems = async ({
    addr
  }: {
    addr: string;
  }): Promise<ArrayOfTupleOfUint64AndListItemData> => {
    return this.client.queryContractSmart(this.contractAddress, {
      owner_un_list_items: {
        addr
      }
    });
  };
  bidList = async ({
    id
  }: {
    id: number;
  }): Promise<ArrayOfTupleOfAddrAndBid> => {
    return this.client.queryContractSmart(this.contractAddress, {
      bid_list: {
        id
      }
    });
  };
}
export interface ContellaMarketInterface extends ContellaMarketReadOnlyInterface {
  contractAddress: string;
  sender: string;
  list: ({
    expires,
    id,
    price
  }: {
    expires: number;
    id: number;
    price: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  removeList: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updatePrice: ({
    id,
    price
  }: {
    id: number;
    price: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setBid: ({
    expires,
    id
  }: {
    expires: number;
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  removeBid: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  buy: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  acceptBid: ({
    bidder,
    id
  }: {
    bidder: string;
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class ContellaMarketClient extends ContellaMarketQueryClient implements ContellaMarketInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.list = this.list.bind(this);
    this.removeList = this.removeList.bind(this);
    this.updatePrice = this.updatePrice.bind(this);
    this.setBid = this.setBid.bind(this);
    this.removeBid = this.removeBid.bind(this);
    this.buy = this.buy.bind(this);
    this.acceptBid = this.acceptBid.bind(this);
  }

  list = async ({
    expires,
    id,
    price
  }: {
    expires: number;
    id: number;
    price: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      list: {
        expires,
        id,
        price
      }
    }, fee, memo, _funds);
  };
  removeList = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_list: {
        id
      }
    }, fee, memo, _funds);
  };
  updatePrice = async ({
    id,
    price
  }: {
    id: number;
    price: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_price: {
        id,
        price
      }
    }, fee, memo, _funds);
  };
  setBid = async ({
    expires,
    id
  }: {
    expires: number;
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_bid: {
        expires,
        id
      }
    }, fee, memo, _funds);
  };
  removeBid = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_bid: {
        id
      }
    }, fee, memo, _funds);
  };
  buy = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      buy: {
        id
      }
    }, fee, memo, _funds);
  };
  acceptBid = async ({
    bidder,
    id
  }: {
    bidder: string;
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      accept_bid: {
        bidder,
        id
      }
    }, fee, memo, _funds);
  };
}