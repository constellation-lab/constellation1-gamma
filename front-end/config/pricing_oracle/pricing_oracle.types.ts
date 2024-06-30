export interface InstantiateMsg {
    admin: string;
    liquidity_pool_contract: string;
    constella_option_contract: string;
  }
  
  export type ExecuteMsg =
    | {
        update_config: {
          admin?: string | null;
          liquidity_pool_contract?: string | null;
          constella_option_contract?: string | null;
        };
      }
    | {
        save_option_price: {
          option_id: number;
          price: Uint128;
        };
      };
  
  export type Uint128 = string;
  
  export type QueryMsg =
    | {
        config: {};
      }
    | {
        calculate_option_price: {
          option_id: number;
          collateral: Uint128;
          counter_offer: Uint128;
          expiration: number;
        };
      }
    | {
        get_option_price: {
          option_id: number;
        };
      }
    | {
        get_pool_info: {};
      };
  
  export interface Config {
    admin: Addr;
    liquidity_pool_contract: Addr;
    constella_option_contract: Addr;
  }
  
  export type Addr = string;
  
  export interface CalculateOptionPriceResponse {
    option_price: Uint128;
  }
  
  export interface OptionPrice {
    option_id: number;
    price: Uint128;
  }
  
  export interface PoolInfo {
    total_collateral: Uint128;
    total_counter_offer: Uint128;
    total_liquidity: Uint128;
  }