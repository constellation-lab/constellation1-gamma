export interface InstantiateMsg {
    name: string;
    symbol: string;
    decimals: number;
    initial_balances: Cw20Coin[];
    mint?: MinterResponse | null;
    marketing?: InstantiateMarketingInfo | null;
  }
  
  export interface Cw20Coin {
    address: string;
    amount: Uint128;
  }
  
  export interface InstantiateMarketingInfo {
    project?: string | null;
    description?: string | null;
    marketing?: string | null;
    logo?: Logo | null;
  }
  
  export type Logo =
    | {
        url: string;
      }
    | {
        embedded: EmbeddedLogo;
      };
  
  export type EmbeddedLogo =
    | {
        svg: Binary;
      }
    | {
        png: Binary;
      };
  
  export type Uint128 = string;
  export type Binary = string;
  
  export interface MinterResponse {
    minter: string;
    cap?: Uint128 | null;
  }
  
  export type ExecuteMsg =
    | {
        transfer: {
          recipient: string;
          amount: Uint128;
        };
      }
    | {
        burn: {
          amount: Uint128;
        };
      }
    | {
        send: {
          contract: string;
          amount: Uint128;
          msg: Binary;
        };
      }
    | {
        mint: {
          recipient: string;
          amount: Uint128;
        };
      }
    | {
        increase_allowance: {
          spender: string;
          amount: Uint128;
          expires?: Expiration | null;
        };
      }
    | {
        decrease_allowance: {
          spender: string;
          amount: Uint128;
          expires?: Expiration | null;
        };
      }
    | {
        transfer_from: {
          owner: string;
          recipient: string;
          amount: Uint128;
        };
      }
    | {
        burn_from: {
          owner: string;
          amount: Uint128;
        };
      }
    | {
        send_from: {
          owner: string;
          contract: string;
          amount: Uint128;
          msg: Binary;
        };
      }
    | {
        stake: {
          amount: Uint128;
        };
      };
  
  export type Expiration =
    | {
        at_height: number;
      }
    | {
        at_time: Timestamp;
      }
    | {
        never: {};
      };
  
  export type Timestamp = Uint64;
  export type Uint64 = string;
  
  export type QueryMsg =
    | {
        balance: {
          address: string;
        };
      }
    | {
        token_info: {};
      }
    | {
        allowance: {
          owner: string;
          spender: string;
        };
      }
    | {
        all_allowances: {
          owner: string;
          start_after?: string | null;
          limit?: number | null;
        };
      }
    | {
        all_accounts: {
          start_after?: string | null;
          limit?: number | null;
        };
      }
    | {
        minter: {};
      }
    | {
        marketing_info: {};
      }
    | {
        download_logo: {};
      };