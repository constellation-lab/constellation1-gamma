export interface InstantiateMsg {
    constella_option_contract: string;
    incentive_manager_contract: string;
    liquidity_pool_contract: string;
    option_marketplace_contract: string;
    owner: string;
    pricing_oracle_contract: string;
  }
  
  export type ExecuteMsg =
    | {
        set_position_limit: {
          max_position: Uint128;
          option_pair: string;
        };
      }
    | {
        set_circuit_breaker: {
          option_pair: string;
          price_threshold: Decimal;
          triggered: boolean;
        };
      }
    | {
        execute_risk_mitigation_strategy: {};
      }
    | {
        adjust_pricing: {
          adjustment_factor: Decimal;
          option_pair: string;
        };
      }
    | {
        close_position: {
          amount: Uint128;
          option_pair: string;
        };
      }
    | {
        adjust_parameters: {
          volatility_multiplier: Decimal;
        };
      };
  
  export type Uint128 = string;
  export type Decimal = string;
  
  export type QueryMsg =
    | {
        config: {};
      }
    | {
        position_limit: {
          option_pair: string;
        };
      }
    | {
        circuit_breaker: {
          option_pair: string;
        };
      }
    | {
        get_pool_info: {};
      }
    | {
        get_option_positions: {};
      }
    | {
        get_price: {
          option_pair: string;
        };
      };
  
  export interface Config {
    constella_option_contract: Addr;
    incentive_manager_contract: Addr;
    liquidity_pool_contract: Addr;
    option_marketplace_contract: Addr;
    owner: Addr;
    pricing_oracle_contract: Addr;
  }
  
  export type Addr = string;
  
  export interface PositionLimit {
    max_position: Uint128;
    option_pair: string;
  }
  
  export interface CircuitBreaker {
    option_pair: string;
    price_threshold: Decimal;
    triggered: boolean;
  }
  
  export interface PoolInfo {
    total_liquidity: Uint128;
  }
  
  export interface OptionPosition {
    amount: Uint128;
    option_pair: string;
  }