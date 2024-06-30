export interface InstantiateMsg {
    assets: Asset[];
  }
  
  export interface Asset {
    denom: string;
    amount: string;
  }
  
  export type ExecuteMsg =
    | {
        deposit: {
          assets: Asset[];
        };
      }
    | {
        withdraw: {
          lp_tokens: string;
        };
      };
  
  export type QueryMsg =
    | {
        get_pool: {};
      }
    | {
        get_liquidity_provider_info: {
          address: string;
        };
      };
  
  export interface LiquidityPool {
    assets: Asset[];
    lp_token_supply: string;
  }
  
  export interface LiquidityProviderInfo {
    address: string;
    assets: Asset[];
    lp_tokens: string;
  }