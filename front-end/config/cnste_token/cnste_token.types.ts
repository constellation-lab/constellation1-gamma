export interface InstantiateMsg {
    constella_option_contract: string;
    incentive_manager_contract: string;
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
        mint: {
          recipient: string;
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
        increase_allowance: {
          spender: string;
          amount: Uint128;
        };
      }
    | {
        decrease_allowance: {
          spender: string;
          amount: Uint128;
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
  
  export type Uint128 = string;
  export type Binary = string;
  
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
      }
    | {
        stake_info: {
          address: string;
        };
      };
  
  export interface StakeInfo {
    staked_amount: Uint128;
  }