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

export interface CreateProposalMsg {
  title: string;
  description: string;
  voting_options: string[];
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
    vote: {
      proposal_id: number;
      vote: boolean;
    };
    }
  | {
      create_proposal: CreateProposalMsg;
    };

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
      voting_power: {
        address: string;
      };
    }
    | {
      proposal: {
        id: number;
      };
    }
    | { 
      list_proposals: { 
        start_after?: number; 
        limit?: number 
      } 
    };

export interface VotingPowerResponse {
  voting_power: Uint128;
}

export interface Proposal {
  id: number;
  title: string;
  description: string;
  status: string;
  voting_options: string[];
  creator: string;
  start_time: number;
  end_time: number;
  total_votes: Uint128;
  // Add any other fields that your proposal structure includes
}

export interface VotingPowerResponse {
  voting_power: Uint128;
}

export interface ListProposalsQuery {
  list_proposals: {
    start_after?: number;
    limit?: number;
  };
}