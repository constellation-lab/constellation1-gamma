import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee, Coin } from "@cosmjs/amino";
import {
  CreateProposalMsg,
  ExecuteMsg,
  QueryMsg,
  Uint128,
  VotingPowerResponse,
  Proposal,
} from "./cdt_token.types";

export interface ProposalListResponse {
  proposals: Proposal[];
}

export class CdtTokenQueryClient {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
  }

  // Query methods

  balance = async (address: string): Promise<Uint128> => {
    const result: Uint128 = await this.client.queryContractSmart(this.contractAddress, {
      balance: { address }
    });
    return result;
  };

  
  votingPower = async (address: string): Promise<VotingPowerResponse> => {
    const result: VotingPowerResponse = await this.client.queryContractSmart(this.contractAddress, {
      voting_power: { address }
    });
    return result;
  };

  // Add the getProposal method
  getProposal = async (id: number): Promise<Proposal> => {
    const result: Proposal = await this.client.queryContractSmart(this.contractAddress, {
      proposal: { id }
    });
    return result;
  };

  listProposals = async (
    params: { start_after?: number; limit?: number } = {}
  ): Promise<ProposalListResponse> => {
    const result: ProposalListResponse = await this.client.queryContractSmart(this.contractAddress, {
      list_proposals: params
    });
    return result;
  };

  // Add other query methods (token_info, allowance, etc.) here
}



export class CdtTokenClient extends CdtTokenQueryClient {
  client: SigningCosmWasmClient;
  sender: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
  }

  // Execute methods

  createProposal = async (
    { title, description, voting_options }: CreateProposalMsg,
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    const msg: ExecuteMsg = {
      create_proposal: { title, description, voting_options }
    };
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      msg,
      fee,
      memo,
      funds
    );
  };

  transfer = async (
    { recipient, amount }: { recipient: string; amount: Uint128 },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    const msg: ExecuteMsg = { transfer: { recipient, amount } };
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      msg,
      fee,
      memo,
      funds
    );
  };

  vote = async (
    {
      proposalId,
      vote,
    }: {
      proposalId: number;
      vote: boolean;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    const msg: ExecuteMsg = {
      vote: { proposal_id: proposalId, vote },
    };
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      msg,
      fee,
      memo,
      funds
    );
  };
}
  // Add other execute methods (burn, mint, send, etc.) here


