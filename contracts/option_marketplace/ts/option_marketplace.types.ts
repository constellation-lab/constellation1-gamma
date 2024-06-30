export interface InstantiateMsg {}

export type ExecuteMsg =
  | {
      list_option: {
        min_trade_amount: Uint128;
        option_id: number;
        price: Uint128;
        slippage_tolerance: Decimal;
      };
    }
  | {
      buy_option: {
        amount: Uint128;
        option_id: number;
      };
    }
  | {
      execute_option: {
        option_id: number;
      };
    };

export type Uint128 = string;
export type Decimal = string;

export type QueryMsg =
  | {
      get_option: {
        option_id: number;
      };
    }
  | {
      list_options: {};
    }
  | {
      get_option_price: {
        option_id: number;
        slippage_tolerance: Decimal;
      };
    };

export interface Option {
  amount: Uint128;
  buyer: Addr;
  is_listed: boolean;
  min_trade_amount: Uint128;
  owner: Addr;
  price: Uint128;
  price_denom: string;
}

export type Addr = string;